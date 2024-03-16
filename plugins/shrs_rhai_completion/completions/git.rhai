fn predicate(ctx){
  let name = ctx.cmd_name;
  name!=()&&name=="git"
}
fn completions(ctx){
  if ctx.arg_num==1{
    return git_comp(ctx)
  }
  return switch ctx.line[1]{
    "add"=> add_comp(ctx),
    "clone"=> clone_comp(ctx),
    "commit"=> commit_comp(ctx),
    "push"=> push_comp(ctx),
    "pull"=> pull_comp(ctx),
    _ => []
  };
}


fn git_comp(ctx){
  if is_short_flag(ctx){
    return with_format([ "-v","-h","-C","-c","-p","-P" ]) ;
  }
  else if is_long_flag(ctx){
    return with_format([["--help", "Show the help message"],
      ["--version", "Show the version information"],
      ["--exec-path=", "Set the Git command search path"],
      ["--html-path", "Print the path where Git's HTML documentation is installed"],
      ["--man-path", "Print the manpath for the manpages"],
      ["--info-path", "Print the path where Git's info files are installed"],
      ["--git-dir=", "Set the path to the repository"],
      ["--work-tree=", "Set the path to the working tree"],
      ["--namespace=", "Set the namespace for Git commands"],
      ["--paginate", "Pipe all output into 'less'"],
      ["--paginate=", "Use the specified pager to view output"],
      ["--no-pager", "Do not paginate output"],
      ["--cd=", "Change to the specified directory before running the Git command"],
      ["--verbose", "Show more information during command execution"],
      ["--quiet", "Suppress all output except errors and fatal messages"],
      ["--dry-run", "Simulate the command without executing it"],
      ["--global", "Apply the option globally to all repositories"],
      ["--local", "Apply the option only to the current repository"],
      ["--system", "Apply the option system-wide"],
      ["--file=", "Specify a configuration file"]]);
  }
  return with_format(["add",
  "am",
  "archive",
  "bisect",
  "branch",
  "bundle",
  "checkout",
  "cherry-pick",
  "citool",
  "clean",
  "clone",
  "commit",
  "describe",
  "diff",
  "fetch",
  "format-patch",
  "gc",
  "grep",
  "gui",
  "init",
  "log",
  "merge",
  "mv",
  "notes",
  "pull",
  "push",
  "rebase",
  "reset",
  "restore",
  "rm",
  "shortlog",
  "show",
  "stash",
  "status",
  "submodule",
  "tag"]);
}
fn clone_comp(ctx) {
    if is_short_flag(ctx) {
        return with_format(["-l","-q","-s","-n"]);
    } else if is_long_flag(ctx) {
        return with_format([
            ["--local", "Bypass the normal 'git aware' transport mechanism and clone the repository from a local repository if possible"],
            ["--quiet", "Operate quietly"],
            ["--bare", "Make a bare Git repository"],
            ["--mirror", "Set up a mirror of the source repository"],
            ["--recursive", "Initialize all submodules in the repository"],
            ["--depth", "Create a shallow clone with a history truncated to the specified number of revisions"],
            ["--shallow-submodules", "All submodules which are cloned will be shallow"],
            ["--no-checkout", "Don't create a local checkout"],
            ["--single-branch", "Clone only a single branch, HEAD or --branch"],
            ["--dissociate", "Clone with --reference but also prevent fetching from the reference"]])
            + 
            with_format([["--template=", "Specify the directory from which templates will be used"],
            ["--reference=", "If the reference repository is on the local machine, automatically setup .git/objects/info/alternates to obtain objects"]],false,Replace());
    }
    // doesnt take into account flag arguments
    let no_flags = filter_flags(ctx.line);
    if no_flags.len==3{
      return with_format(["https://github.com/","git@github.com:","https://gitlab.com/","git@gitlab.com:"],false,Replace());
    }
    if no_flags.len==4{
      return filename_completions(ctx);
    }
    []

}

fn add_comp(ctx){
  if is_short_flag(ctx) {
      return with_format(["-n", "-v", "-f", "-i", "-p"]);
  } else if is_long_flag(ctx) {
      return with_format([
          ["--dry-run", "Simulate adding files without modifying the index"],
          ["--verbose", "Be verbose"],
          ["--force", "Allow adding otherwise ignored files"],
          ["--interactive", "Add modified contents interactively"],
          ["--patch", "Interactively choose hunks of patch between the index and the work tree"]
      ]);
  }

  filename_completions(ctx)+[Completion(".")]
}
fn commit_comp(ctx) {
    if is_short_flag(ctx) {
        return default_format(["-m", "-a", "-s", "-v"]);
    } else if is_long_flag(ctx) {
        return with_format([
            ["--message", "Use the given <msg> as the commit message"],
            ["--all", "Commit all files changed"],
            ["--signoff", "Add Signed-off-by line"],
            ["--verbose", "Show unified diff of all file changes"],
        ]);
    }
    []
}

fn push_comp(ctx) {
    if is_short_flag(ctx) {
        return with_format(["-f", "-v", "-u", "-n"]);
    } else if is_long_flag(ctx) {
        return with_format([
            ["--force", "Force updates"],
            ["--verbose", "Be verbose"],
            ["--set-upstream", "Set up 'origin' as upstream"],
            ["--dry-run", "Do everything except actually send the updates"],
        ]);
    }
    []
}

fn pull_comp(ctx) {
    if is_short_flag(ctx) {
        return with_format(["-f", "-v", "-r", "-a"]);
    } else if is_long_flag(ctx) {
        return with_format([
            ["--force", "Force updates"],
            ["--verbose", "Be verbose"],
            ["--rebase", "Rebase instead of merge"],
            ["--all", "Fetch all remotes"],
        ]);
    }
    []
}





