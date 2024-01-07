//! Collection of completion functions

use std::path::{Path, PathBuf};

// also provide some commonly used completion lists
// - directories
// - executables
// - file extension
// - filename regex
// - known hosts

// SWAP, a lot of time we need a bunch more context, like the shell's env or the current working
// directory, consider what we can do to have completer functions that need 'initalizion'.

/// Generate list of files in the current working directory with predicate
pub(crate) fn filepaths_p<P>(dir: &Path, predicate: P) -> std::io::Result<Vec<PathBuf>>
where
    P: FnMut(&std::fs::DirEntry) -> bool,
{
    use std::fs;

    let out: Vec<PathBuf> = fs::read_dir(dir)?
        .filter_map(|f| f.ok())
        .filter(predicate)
        .map(|f| f.path())
        .collect();

    Ok(out)
}

/// Generate list of files in the current working directory
pub(crate) fn filepaths(dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    filepaths_p(dir, |_| true)
}

/// Looks through each directory in path and finds executables
pub(crate) fn find_executables_in_path(path_str: &str) -> Vec<String> {
    use std::{fs, os::unix::fs::PermissionsExt};

    let mut execs = vec![];
    for path in path_str.split(':') {
        let dir = match fs::read_dir(path) {
            Ok(dir) => dir,
            Err(_) => continue,
        };
        for dir_entry in dir.flatten() {
            // check if file is executable
            if dir_entry.metadata().unwrap().permissions().mode() & 0o111 != 0 {
                execs.push(dir_entry.file_name().to_str().unwrap().into());
            }
        }
    }
    execs
}

/// Drop everything after the last / character
pub(crate) fn drop_path_end(path: &str) -> String {
    let drop_end = path
        .chars()
        .rev()
        .skip_while(|c| *c != '/')
        .collect::<String>();
    drop_end.chars().rev().collect::<String>()
}

// pub(crate) fn path_end(path: &str) -> String {
//     let end = path
//         .chars()
//         .rev()
//         .take_while(|c| *c != '/')
//         .collect::<String>();
//     end.chars().rev().collect::<String>()
// }

#[cfg(test)]
mod tests {
    use super::drop_path_end;

    #[test]
    fn test_drop_path_end() {
        assert_eq!(drop_path_end("Downloads/ab"), "Downloads/".to_owned());
        assert_eq!(drop_path_end("Downloads/"), "Downloads/".to_owned());
        assert_eq!(drop_path_end("Downloads"), "".to_owned());
    }
}
