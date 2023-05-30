use std::{
    ffi::OsStr,
    fmt, iter,
    os::fd::AsRawFd,
    process::{Child, ChildStdout, Command, ExitStatus, Stdio},
};

use nix::libc::{STDERR_FILENO, STDIN_FILENO, STDOUT_FILENO};

use super::{io::Stdin, pid_t, util, Output};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProcessId(u32);

impl From<u32> for ProcessId {
    fn from(value: u32) -> Self {
        ProcessId(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProcessStatus {
    Running,
    Stopped,
    Completed,
}

pub trait Process {
    fn id(&self) -> Option<ProcessId>;
    fn argv(&self) -> String;
    fn status(&self) -> ProcessStatus;
    fn status_code(&self) -> Option<ExitStatus>;
    fn stdout(&mut self) -> Option<Stdin>;
    fn kill(&mut self) -> anyhow::Result<()>;
    fn wait(&mut self) -> anyhow::Result<ExitStatus>;
    fn try_wait(&mut self) -> anyhow::Result<Option<ExitStatus>>;
}

impl fmt::Debug for dyn Process {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Process {{ id: {} }}",
            self.id()
                .map(|id| id.0.to_string())
                .unwrap_or_else(|| "(builtin)".to_string())
        )
    }
}

#[derive(Debug)]
pub struct ProcessGroup {
    pub id: Option<u32>,
    pub processes: Vec<Box<dyn Process>>,
    pub foreground: bool,
}

struct BuiltinProcess {
    argv: Vec<String>,
    status_code: ExitStatus,
    stdout: Option<Stdin>,
}

impl BuiltinProcess {
    pub fn new<S1, S2>(
        program: S1,
        args: &[S2],
        status_code: ExitStatus,
        stdout: Option<Stdin>,
    ) -> Self
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        Self {
            argv: iter::once(program)
                .map(|p| p.as_ref().to_string())
                .chain(args.iter().map(|arg| arg.as_ref().to_string()))
                .collect(),
            status_code,
            stdout,
        }
    }
}

impl Process for BuiltinProcess {
    fn id(&self) -> Option<ProcessId> {
        None
    }

    fn argv(&self) -> String {
        self.argv[..].join(" ")
    }

    fn status(&self) -> ProcessStatus {
        ProcessStatus::Completed
    }

    fn status_code(&self) -> Option<ExitStatus> {
        Some(self.status_code)
    }

    fn stdout(&mut self) -> Option<Stdin> {
        self.stdout.take()
    }

    fn kill(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn wait(&mut self) -> anyhow::Result<ExitStatus> {
        Ok(self.status_code)
    }

    fn try_wait(&mut self) -> anyhow::Result<Option<ExitStatus>> {
        Ok(Some(self.status_code))
    }
}

struct ExternalProcess {
    argv: Vec<String>,
    child: Child,
    status: ProcessStatus,
    status_code: Option<ExitStatus>,
}

impl ExternalProcess {
    pub fn new<S1, S2>(program: S1, args: &[S2], child: Child) -> Self
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        Self {
            argv: iter::once(&program)
                .map(|p| p.as_ref().to_string())
                .chain(args.iter().map(|arg| arg.as_ref().to_string()))
                .collect(),
            child,
            status: ProcessStatus::Running,
            status_code: None,
        }
    }
}

impl Process for ExternalProcess {
    fn id(&self) -> Option<ProcessId> {
        Some(self.child.id().into())
    }

    fn argv(&self) -> String {
        self.argv[..].join(" ")
    }

    fn status(&self) -> ProcessStatus {
        self.status
    }

    fn status_code(&self) -> Option<ExitStatus> {
        self.status_code
    }

    fn stdout(&mut self) -> Option<Stdin> {
        self.child.stdout.take().map(Stdin::Child)
    }

    fn kill(&mut self) -> anyhow::Result<()> {
        self.child.kill()?;
        Ok(())
    }

    fn wait(&mut self) -> anyhow::Result<ExitStatus> {
        let exit_status = self.child.wait()?;
        self.status = ProcessStatus::Completed;
        self.status_code = Some(exit_status);
        Ok(exit_status)
    }

    fn try_wait(&mut self) -> anyhow::Result<Option<ExitStatus>> {
        if let Some(exit_status) = self.child.try_wait()? {
            self.status = ProcessStatus::Completed;
            self.status_code = Some(exit_status);
            Ok(Some(exit_status))
        } else {
            Ok(None)
        }
    }
}

pub fn run_external_command<S1, S2>(
    program: S1,
    args: &[S2],
    stdin: Stdin,
    stdout: Output,
    stderr: Output,
    pgid: Option<u32>,
) -> anyhow::Result<(Box<dyn Process>, Option<u32>)>
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    use std::os::unix::process::CommandExt;

    use nix::{
        sys::signal::{self, SigHandler, Signal},
        unistd::{self, Pid},
    };

    let mut command = Command::new(OsStr::new(program.as_ref()));
    command.args(args.iter().map(AsRef::as_ref).map(OsStr::new));

    // Configure stdout and stderr (e.g. pipe, redirect). Do not configure
    // stdin, as we need to do that manually in before_exec *after* we have
    // set the terminal control device to the job's process group. If we were
    // to configure stdin here, then stdin would be changed before our code
    // executes in before_exec, so if the child is not the first process in the
    // pipeline, its stdin would not be a tty and tcsetpgrp would tell us so.
    let stdout_fd = if let Output::FileDescriptor(fd) = stdout {
        Some(fd)
    } else {
        command.stdout(stdout);
        None
    };

    let stderr_fd = if let Output::FileDescriptor(fd) = stderr {
        Some(fd)
    } else {
        command.stderr(stderr);
        None
    };

    // TODO add abiliity to disable job control
    // let job_control_is_enabled = shell.is_job_control_enabled();
    let job_control_is_enabled = true;
    let shell_terminal = util::get_terminal();
    unsafe {
        command.pre_exec(move || {
            if job_control_is_enabled {
                // Put process into process group
                let pid = unistd::getpid();
                let pgid = pgid.map(|pgid| Pid::from_raw(pgid as i32)).unwrap_or(pid);

                // setpgid(2) failing represents programmer error, e.g.
                // 1) invalid pid or pgid
                unistd::setpgid(pid, pgid).expect("setpgid failed");

                // Set the terminal control device in both parent process (see job
                // manager) and child process to avoid race conditions
                // tcsetpgrp(3) failing represents programmer error, e.g.
                // 1) invalid fd or pgid
                // 2) not a tty
                //   - Are you configuring stdin using Command::stdin? If so, then
                //     stdin will not be a TTY if this process isn't first in the
                //     pipeline, as Command::stdin configures stdin *before*
                //     before_exec runs.
                // 3) incorrect permissions
                unistd::tcsetpgrp(shell_terminal, pgid).expect("tcsetpgrp failed");

                // Reset job control signal handling back to default
                // signal(3) failing represents programmer error, e.g.
                // 1) signal argument is not a valid signal number
                // 2) an attempt is made to supply a signal handler for a
                //    signal that cannot have a custom signal handler
                for signal in [
                    Signal::SIGINT,
                    Signal::SIGQUIT,
                    Signal::SIGTSTP,
                    Signal::SIGTTIN,
                    Signal::SIGTTOU,
                    Signal::SIGCHLD,
                ] {
                    signal::signal(signal, SigHandler::SigDfl)
                        .expect("failed to reset signal handler");
                }
            }

            // See comment at the top of this function on why we are configuring
            // this manually (hint: it's because tcsetpgrp needs the original stdin
            // and Command::stdin will change stdin *before* before_exec runs).
            let stdin = stdin.as_raw_fd();
            if stdin != STDIN_FILENO {
                unistd::dup2(stdin, STDIN_FILENO).expect("failed to dup stdin");
                unistd::close(stdin).expect("failed to close stdin");
            }

            if let Some(fd) = stdout_fd {
                if fd != STDOUT_FILENO {
                    unistd::dup2(fd, STDOUT_FILENO).expect("failed to dup stdout");
                    unistd::close(fd).expect("failed to close stdout");
                }
            }

            if let Some(fd) = stderr_fd {
                if fd != STDERR_FILENO {
                    unistd::dup2(fd, STDERR_FILENO).expect("failed to dup stderr");
                    unistd::close(fd).expect("failed to close stderr");
                }
            }

            Ok(())
        });
    }

    let child = match command.spawn() {
        Ok(child) => child,
        Err(e) => {
            if job_control_is_enabled {
                // use log::warn;
                // warn!("failed to spawn child, resetting terminal's pgrp");
                // see above comment for tcsetpgrp(2) failing being programmer
                // error
                unistd::tcsetpgrp(util::get_terminal(), unistd::getpgrp()).unwrap();
            }

            return Err(e.into());
        },
    };

    let pgid = pgid.unwrap_or_else(|| child.id());
    if job_control_is_enabled {
        let temp_result = unistd::setpgid(
            Pid::from_raw(child.id() as pid_t),
            Pid::from_raw(pgid as pid_t),
        );

        // log_if_err!(
        //     temp_result,
        //     "failed to set pgid ({}) for pid ({})",
        //     child.id(),
        //     pgid
        // );
    }

    Ok((
        Box::new(ExternalProcess::new(program, args, child)),
        Some(pgid),
    ))
}

/*
fn run_builtin_command<S1, S2>(
    shell: &mut dyn Shell,
    program: S1,
    args: &[S2],
    stdout: Output,
    pgid: Option<u32>,
) -> Result<(Box<dyn Process>, Option<u32>)>
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    // TODO(rogardn): change Result usage in builtin to only be for rust
    // errors, e.g. builtin::execute shouldn't return a Result
    let (status_code, output) = match stdout {
        Output::File(mut file) => (builtins::run(shell, &program, args, &mut file).0, None),
        Output::FileDescriptor(_fd) => unimplemented!(),
        Output::CreatePipe => {
            let (read_end_pipe, mut write_end_pipe) = create_pipe()?;
            (
                builtins::run(shell, &program, args, &mut write_end_pipe).0,
                Some(read_end_pipe.into()),
            )
        }
        Output::Inherit => (
            builtins::run(shell, &program, args, &mut io::stdout()).0,
            None,
        ),
    };

    Ok((
        Box::new(BuiltinProcess::new(&program, args, status_code, output)),
        pgid,
    ))
}
*/
