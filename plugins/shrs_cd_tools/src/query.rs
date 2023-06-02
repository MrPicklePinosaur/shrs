//! Scan file system to match project type

use std::{ffi::OsString, fs, path::Path};

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct Query {
    /// Required files (exact match)
    #[builder(default = "Vec::new()")]
    files: Vec<String>,
    /// Required file extensions
    #[builder(default = "Vec::new()")]
    extensions: Vec<String>,
    /// Required directories
    #[builder(default = "Vec::new()")]
    dirs: Vec<String>,

    /// Should query be performed recursively
    #[builder(default = "true")]
    recursive: bool,
}

impl Query {
    /// Runs filesystem query and returns if query matched
    pub fn scan(&self, dir: &Path) -> bool {
        // TODO run this recursively
        // look for required files
        let found_files = self.files.iter().all(|required_file| {
            let mut dir_contents = fs::read_dir(dir).unwrap();
            dir_contents.any(|f| f.as_ref().unwrap().file_name() == OsString::from(required_file))
        });

        // look for required file extensions

        // look for required dirs

        found_files
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::QueryBuilder;

    #[test]
    fn basic() {
        let query = QueryBuilder::default()
            .files(vec![String::from(".vimrc")])
            .build()
            .unwrap();

        // TODO make proper test (that works on all dev machines)
        let path = PathBuf::from("/home/pinosaur");
        assert!(query.scan(&path));
    }
}
