

fn predicate(line){
  let name = line.cmd_name;
  name!=()&&name=="git"&&line.arg_num==1
}
fn completions(line){
  if is_short_flag(line){
    return default_format([ "-v","-h","-C","-c","-p","-P" ]) ;
  }
  else if is_long_flag(line){
    return default_format_with_comment(
[
    ["--help", "Show the help message"],
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
    ["--file=", "Specify a configuration file"]
]
    );
  }


  return default_format(["add",
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

