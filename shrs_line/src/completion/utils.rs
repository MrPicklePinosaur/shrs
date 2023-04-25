//! Collection of completion functions

use std::path::Path;

/// Generate list of files in the current working directory with predicate
pub fn filepath_completion_p<P>(dir: &Path, predicate: P) -> std::io::Result<Vec<String>>
where
    P: FnMut(&std::fs::DirEntry) -> bool,
{
    use std::fs;

    let out: Vec<String> = fs::read_dir(dir)?
        .filter_map(|f| f.ok())
        .filter(predicate)
        .map(|f| f.file_name().into_string())
        .filter_map(|f| f.ok())
        .collect();

    Ok(out)
}

/// Generate list of files in the current working directory
pub fn all_files_completion(dir: &Path) -> std::io::Result<Vec<String>> {
    filepath_completion_p(dir, |_| true)
}

/// Generate list of all executables in PATH
pub fn exectuable_completion(_dir: &Path) -> std::io::Result<Vec<String>> {
    todo!()
}

/// Generate list of all ssh hosts
pub fn ssh_completion(_dir: &Path) -> std::io::Result<Vec<String>> {
    todo!()
}
