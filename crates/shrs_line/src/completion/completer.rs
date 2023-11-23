//! Implementation of default rule based completer

use std::path::{Path, PathBuf};

use shrs_core::builtin::Builtins;

use super::{
    drop_path_end, filepaths, find_executables_in_path, Completer, Completion, CompletionCtx,
    ReplaceMethod,
};

// TODO make this FnMut?
/// Actions return a list of possible completions
pub type Action = Box<dyn Fn(&CompletionCtx) -> Vec<Completion>>;

/// Predicates are functions that take in some context and return a boolean
pub struct Pred {
    pred: Box<dyn Fn(&CompletionCtx) -> bool>,
}

impl Pred {
    /// Construct a new predicate from a function
    pub fn new(pred: impl Fn(&CompletionCtx) -> bool + 'static) -> Self {
        Self {
            pred: Box::new(pred),
        }
    }
    /// Chain another predicate
    ///
    /// The predicates short circuit
    pub fn and(self, pred: impl Fn(&CompletionCtx) -> bool + 'static) -> Self {
        Self {
            pred: Box::new(move |ctx: &CompletionCtx| -> bool { (*self.pred)(ctx) && pred(ctx) }),
        }
    }
    /// Check if predicate is satisfied
    pub fn test(&self, ctx: &CompletionCtx) -> bool {
        (self.pred)(ctx)
    }
}

pub type Filter = Box<dyn Fn(&String) -> bool>;
pub type Format = Box<dyn Fn(String) -> Completion>;

/// Rules for [DefaultCompleter]
pub struct Rule {
    /// Predicate to check
    pub pred: Pred,
    /// Action to execute if predicate is satisfied
    pub action: Action,
    // pub filter: Filter,
    // pub format: Format,
}

impl Rule {
    pub fn new(pred: Pred, action: impl Fn(&CompletionCtx) -> Vec<Completion> + 'static) -> Self {
        Self {
            pred,
            action: Box::new(action),
            // filter:
            // format: Box::new(default_format),
        }
    }

    // TODO this could maybe be rewritten as a builder pattern
}

/// Default rule-based completion system
pub struct DefaultCompleter {
    rules: Vec<Rule>,
}

impl DefaultCompleter {
    pub fn new() -> Self {
        Self { rules: vec![] }
    }

    /// Register a new rule to use
    pub fn register(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    fn complete_helper(&self, ctx: &CompletionCtx) -> Vec<Completion> {
        let rules = self.rules.iter().filter(|p| (p.pred).test(ctx));

        let mut output = vec![];
        for rule in rules {
            // if rule was matched, run the corresponding action
            // also do prefix search (could make if prefix search is used a config option)
            let mut comps = (rule.action)(ctx)
                .into_iter()
                .filter(|s| {
                    s.accept()
                        .starts_with(ctx.cur_word().unwrap_or(&String::new()))
                })
                // .map(|s| (rule.format)(s))
                .collect::<Vec<_>>();

            output.append(&mut comps);
        }
        output
    }
}

impl Completer for DefaultCompleter {
    fn complete(&self, ctx: &CompletionCtx) -> Vec<Completion> {
        self.complete_helper(ctx)
    }
}

impl Default for DefaultCompleter {
    /// Register default rules
    fn default() -> Self {
        // collection of predefined rules

        let mut comp = DefaultCompleter::new();
        comp.register(Rule::new(
            Pred::new(git_pred).and(flag_pred),
            Box::new(git_flag_action),
        ));
        comp.register(Rule::new(Pred::new(git_pred), Box::new(git_action)));
        comp.register(Rule::new(
            Pred::new(ls_pred).and(short_flag_pred),
            Box::new(ls_short_flag_action),
        ));
        comp.register(Rule::new(
            Pred::new(ls_pred).and(long_flag_pred),
            Box::new(ls_long_flag_action),
        ));
        comp.register(Rule::new(Pred::new(arg_pred), Box::new(filename_action)));
        comp.register(Rule::new(Pred::new(arg_pred), Box::new(filename_action)));
        comp
    }
}

/// Return all the executables in PATH
pub fn cmdname_action(path_str: String) -> impl Fn(&CompletionCtx) -> Vec<Completion> {
    move |_ctx: &CompletionCtx| -> Vec<Completion> {
        default_format(find_executables_in_path(&path_str))
    }
}

/// Return all the builtin command names
pub fn builtin_cmdname_action(builtin: &Builtins) -> impl Fn(&CompletionCtx) -> Vec<Completion> {
    let builtin_names = builtin
        .iter()
        .map(|(name, _)| name.to_owned().to_string())
        .collect::<Vec<_>>();
    move |_ctx: &CompletionCtx| -> Vec<Completion> { default_format(builtin_names.clone()) }
}

/// Look in current directory for potential filenames to complete
pub fn filename_action(ctx: &CompletionCtx) -> Vec<Completion> {
    let cur_word = ctx.cur_word().unwrap();
    let drop_end = drop_path_end(cur_word);
    let cur_path = to_absolute(&drop_end, &dirs::home_dir().unwrap());

    let output = filepaths(&cur_path).unwrap_or(vec![]);
    output
        .iter()
        .map(|x| {
            let filename = x.file_name().unwrap().to_str().unwrap().to_string();

            let mut filename = sanitize_file_name(filename);

            // escape special characters in filename

            // append slash if directory name
            let is_dir = x.is_dir();
            if is_dir {
                filename += "/";
            }
            Completion {
                add_space: !is_dir,
                display: Some(filename.to_owned()),
                completion: drop_end.to_owned() + &filename,
                replace_method: ReplaceMethod::Replace,
                comment: None,
            }
        })
        .collect::<Vec<_>>()
}

/// Takes in an arbitrary path that user enters and convert it into an absolute path
fn to_absolute(path_str: &str, home_dir: &Path) -> PathBuf {
    let path_buf = PathBuf::from(path_str);

    let absolute = if path_buf.has_root() {
        path_buf
    } else {
        // handle home directory tilde
        // TODO ~username/ not yet handled
        if let Ok(stripped) = path_buf.strip_prefix("~/") {
            home_dir.join(stripped)
        } else {
            std::env::current_dir().unwrap().join(path_buf)
        }
    };

    absolute
}

pub fn git_action(_ctx: &CompletionCtx) -> Vec<Completion> {
    default_format(vec!["status".into(), "add".into(), "commit".into()])
}

pub fn git_flag_action(_ctx: &CompletionCtx) -> Vec<Completion> {
    default_format(vec!["--version".into(), "--help".into(), "--bare".into()])
}

/// Check if we are completing the command name
pub fn cmdname_pred(ctx: &CompletionCtx) -> bool {
    ctx.arg_num() == 0
}
pub fn git_pred(ctx: &CompletionCtx) -> bool {
    cmdname_eq_pred("git".into())(ctx)
}

/// Check if we are attempting to complete an argument
pub fn arg_pred(ctx: &CompletionCtx) -> bool {
    ctx.arg_num() != 0
}

/// Check if name of current command equals a given command name
pub fn cmdname_eq_pred(cmd_name: String) -> impl Fn(&CompletionCtx) -> bool {
    move |ctx: &CompletionCtx| ctx.cmd_name() == Some(&cmd_name)
}

/// Check if we are completing a flag
pub fn flag_pred(ctx: &CompletionCtx) -> bool {
    long_flag_pred(ctx) || short_flag_pred(ctx)
}
/// Check if we are completing a short flag
pub fn short_flag_pred(ctx: &CompletionCtx) -> bool {
    ctx.cur_word().unwrap_or(&String::new()).starts_with('-') && !long_flag_pred(ctx)
}
/// Check if we are completing a long flag
pub fn long_flag_pred(ctx: &CompletionCtx) -> bool {
    ctx.cur_word().unwrap_or(&String::new()).starts_with("--")
}

/// Check if we are completing a (real) path
pub fn path_pred(ctx: &CompletionCtx) -> bool {
    // strip part after slash
    let cur_word = ctx.cur_word().unwrap();
    // TODO should technically be using HOME env variable?
    let cur_path = to_absolute(&drop_path_end(cur_word), &dirs::home_dir().unwrap());

    cur_path.is_dir()
}

// TODO temp helper to create a list of completions
/// Construct a [Completion] with default options
pub fn default_format(s: Vec<String>) -> Vec<Completion> {
    s.iter()
        .map(|x| Completion {
            add_space: true,
            display: None,
            completion: x.to_owned(),
            replace_method: ReplaceMethod::Replace,
            comment: None,
        })
        .collect::<Vec<_>>()
}

pub fn default_format_with_comment(s: Vec<(&'static str, &'static str)>) -> Vec<Completion> {
    s.iter()
        .map(|x| Completion {
            add_space: true,
            display: None,
            completion: x.0.to_string(),
            replace_method: ReplaceMethod::Replace,
            comment: Some(x.1.to_string()),
        })
        .collect::<Vec<_>>()
}

// pub fn path_format(s: String) -> Completion {
//     Completion { add_space: false, display: Some(path_end(&s)), completion: s }
// }

fn sanitize_file_name(filename: String) -> String {
    // lazy_static! {
    //     static ref ESCAPE_SPACE
    // }

    filename.replace(" ", "\\ ")
}

// completions for ls command
pub fn ls_pred(ctx: &CompletionCtx) -> bool {
    cmdname_eq_pred("ls".into())(ctx)
}

pub fn ls_short_flag_action(_ctx: &CompletionCtx) -> Vec<Completion> {
    default_format(
        vec![
            "-a", "-A", "-b", "-B", "-c", "-C", "-d", "-D", "-f", "-F", "-g", "-G", "-h", "-H",
            "-i", "-I", "-k", "-l", "-L", "-m", "-n", "-N", "-o", "-p", "-q", "-Q", "-r", "-R",
            "-s", "-S", "-t", "-T", "-u", "-U", "-v", "-w", "-x", "-X", "-Z", "-1",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>(),
    )
}

pub fn ls_long_flag_action(_ctx: &CompletionCtx) -> Vec<Completion> {
    default_format_with_comment(
        vec![
            ("--all", "do not ignore entries starting with ."),
            ("--almost-all", "do not list implied . and .."),
            ("--author", "with -l, print the author of each file"),
            ("--escape", "print C-style escapes for nongraphic characters"),
            ("--block-size", "with -l, scale sizes by SIZE when printing them; e.g., '--block-size=M'; see SIZE format below"),
            ("--ignore-backups", "do not list implied entries ending with ~"),
            ("--color", "colorize the output; WHEN can be 'always' (default if omitted), 'auto', or 'never'; more info below"),
            ("--directory", "list directories themselves, not their contents"),
            ("--dired", "generate output designed for Emacs' dired mode"),
            ("--classify", "append indicator (one of */=>@|) to entries"),
            ("--file-type", "likewise, except do not append '*'"),
            ("--format", "across -x, commas -m, horizontal -x, long -l, single-column -1, verbose -l, vertical -C"),
            ("--full-time", "like -l --time-style=full-iso"),
            ("--group-directories-first", "group directories before files; can be augmented with a --sort option, but any use of --sort=none (-U) disables grouping"),
            ("--no-group", "in a long listing, don't print group names"),
            ("--human-readable", "with -l and -s, print sizes like 1K 234M 2G etc."),
            ("--si", "likewise, but use powers of 1000 not 1024"),
            ("--dereference-command-line", "follow symbolic links listed on the command line"),
            ("--dereference-command-line-symlink-to-dir", "follow each command line symbolic link that points to a directory"),
            ("--hide", "do not list implied entries matching shell PATTERN (overridden by -a or -A)"),
            ("--hyperlink", "hyperlink file names; WHEN can be 'always' (default if omitted), 'auto', or 'never'"),
            ("--indicator-style", "append indicator with style WORD to entry names: none (default), slash (-p), file-type (--file-type), classify (-F)"),
            ("--inode", "print the index number of each file"),
            ("--ignore", "do not list implied entries matching shell PATTERN"),
            ("--kibibytes", "default to 1024-byte blocks for disk usage; used only with -s and per directory totals -l                         use a long listing format"),
            ("--dereference", "when showing file information for a symbolic link, show information for the file the link references rather than for the link itself"),
            ("--numeric-uid-gid", "like -l, but list numeric user and group IDs"),
            ("--literal", "print entry names without quoting"),
            ("--indicator-style=slash", "append / indicator to directories"),
            ("--hide-control-chars", "print ? instead of nongraphic characters"),
            ("--show-control-chars", "show nongraphic characters as-is (the default, unless program is 'ls' and output is a terminal)"),
            ("--quote-name", "enclose entry names in double quotes"),
            ("--quoting-style", "use quoting style WORD for entry names: literal, locale, shell, shell-always, shell-escape, shell-escape-always, c, escape (overrides QUOTING_STYLE environment variable)"),
            ("--reverse", "reverse order while sorting"),
            ("--recursive", "list subdirectories recursively"),
            ("--size", "print the allocated size of each file, in blocks"),
            ("--sort", "sort by WORD instead of name: none (-U), size (-S), time (-t), version (-v), extension (-X)"),
            ("--time", "change the default of using modification times; access time (-u): atime, access, use; change time (-c): ctime, status; birth time: birth, creation; with -l, WORD determines which time to show; with --sort=time, sort by WORD (newest first)"),
            ("--time-style", "time/date format with -l; see TIME_STYLE below"),
            ("--tabsize", "assume tab stops at each COLS instead of 8"),
            ("--width", "set output width to COLS.  0 means no limit"),
            ("--context", "print any security context of each file"),
            ("--help", "display this help and exit"),
            ("--version", "output version information and exit"),
        ]
    )
}

#[cfg(test)]
mod tests {
    use super::{flag_pred, DefaultCompleter};
    use crate::completion::CompletionCtx;

    #[test]
    fn simple() {
        let _comp = DefaultCompleter::new();
        // comp.register(Rule::new());
    }

    #[test]
    fn test_is_flag() {
        let ctx = CompletionCtx::new(vec!["git".into(), "-".into()]);
        assert!(flag_pred(&ctx));
        let ctx = CompletionCtx::new(vec![]);
        assert!(!flag_pred(&ctx));
    }
}
