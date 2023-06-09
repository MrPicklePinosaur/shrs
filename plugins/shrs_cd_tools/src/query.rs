//! Scan file system to match project type

use std::{any::TypeId, ffi::OsString, fs, marker::PhantomData, path::Path};

use anymap::AnyMap;
use multimap::MultiMap;
use serde::{Deserialize, DeserializeOwned, Serialize};
use serde_json::Value;
use shrs::anyhow;

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

    /// List of parsers for metadata
    #[builder(default = "MultiMap::new()")]
    metadata_parsers: MultiMap<String, MetadataParser>,
}

/// Return information about the file directory scan
pub struct QueryResult {
    pub matched: bool,
    pub metadata: AnyMap,
}

/// How to parse metadata
pub type MetadataParser = Box<dyn Fn(String) -> Value>;

impl Query {
    /// Runs filesystem query and returns if query matched
    pub fn scan(&self, dir: &Path) -> QueryResult {
        let mut metadata = AnyMap::new();

        // TODO run this recursively
        // look for required files
        let found_files = self.files.iter().all(|required_file| {
            let mut dir_contents = fs::read_dir(dir).unwrap();
            dir_contents.any(|f| f.as_ref().unwrap().file_name() == OsString::from(required_file))
        });

        // TODO redundant code
        let mut dir_contents = fs::read_dir(dir).unwrap();
        for dir_item in dir_contents.into_iter() {
            let dir_item = dir_item.unwrap();
            if dir_item.file_type().unwrap().is_file() {
                let file_name: String = dir_item.file_name().to_string_lossy().into();
                if let Some(inner) = self.metadata_parsers.get_vec(&file_name) {
                    for parser in inner {
                        let res = (*parser)(String::new());
                    }
                }
            }
        }

        // look for required file extensions

        // look for required dirs

        QueryResult {
            matched: found_files,
            metadata,
        }
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
        assert!(query.scan(&path).matched);
    }
}
