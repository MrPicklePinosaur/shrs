//! Collection of useful predicates and actions

use super::{
    cmdname_eq_pred, default_format, default_format_with_comment, Completion, CompletionCtx,
};

// completions for git
pub fn git_action(_ctx: &CompletionCtx) -> Vec<Completion> {
    default_format(vec!["status".into(), "add".into(), "commit".into()])
}

pub fn git_flag_action(_ctx: &CompletionCtx) -> Vec<Completion> {
    default_format(vec!["--version".into(), "--help".into(), "--bare".into()])
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
