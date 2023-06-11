//! Scan file system to match project type

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    ffi::OsString,
    fs,
    marker::PhantomData,
    path::Path,
};

use anymap::AnyMap;
use multimap::MultiMap;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
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
    #[builder(default = "HashMap::new()")]
    metadata_parsers: HashMap<String, MetadataParser>,
}

/// How to match a file
pub enum FileMatcher {
    /// Match a file based only on it's name
    ///
    /// TODO this can also be a regex
    Filename(String),
}

/// Return information about the file directory scan
pub struct QueryResult {
    pub matched: bool,
    pub metadata: AnyMap,
}

impl QueryResult {
    pub fn new() -> Self {
        Self {
            matched: false,
            metadata: AnyMap::new(),
        }
    }

    pub fn add_metadata<T: 'static>(&mut self, data: T) {
        self.metadata.insert(data);
    }
    pub fn get_metadata<T: 'static>(&self) -> Option<&T> {
        self.metadata.get::<T>()
    }
}

/// How to parse metadata
///
/// This handler is responsible for inserting the metadata into the metadata map
/// The current reason for this is that it's a limitation with the type system (or limitation of my
/// abilities). Hopefully will come up with more ergonomic solution in the future
pub type MetadataParser = Box<dyn Fn(&mut QueryResult, &String) -> anyhow::Result<()>>;

impl Query {
    /// Runs filesystem query and returns if query matched
    pub fn scan(&self, dir: &Path) -> QueryResult {
        let mut query_res = QueryResult {
            matched: false,
            metadata: AnyMap::new(),
        };

        // TODO run this recursively
        // look for required files
        let found_files = self.files.iter().all(|required_file| {
            let mut dir_contents = fs::read_dir(dir).unwrap();
            dir_contents.any(|f| f.as_ref().unwrap().file_name() == OsString::from(required_file))
        });

        // TODO redundant code
        let dir_contents = fs::read_dir(dir).unwrap();
        for dir_item in dir_contents.into_iter() {
            let dir_item = dir_item.unwrap();
            if dir_item.file_type().unwrap().is_file() {
                let file_name: String = dir_item.file_name().to_string_lossy().into();
                let file_path = dir_item.path();

                // run parser
                if let Some(parser) = self.metadata_parsers.get(&file_name) {
                    let contents = fs::read_to_string(file_path).unwrap();
                    let res = (*parser)(&mut query_res, &contents);
                    // TODO warn or handle error if parser errors
                    if let Err(e) = res {
                        eprintln!("{:?}", e);
                    }
                }
            }
        }

        query_res.matched = found_files;
        query_res
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::PathBuf};

    use serde::Deserialize;
    use shrs::anyhow;

    use super::{MetadataParser, QueryBuilder, QueryResult};

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

    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct TestParse {
        ip: String,
        port: Option<u16>,
    }

    fn parser(query_res: &mut QueryResult, content: &String) -> anyhow::Result<()> {
        let parsed: TestParse = toml::from_str(content)?;
        query_res.add_metadata(parsed);
        Ok(())
    }

    #[test]
    fn metadata_parse() -> anyhow::Result<()> {
        let mut query_res = QueryResult::new();
        let test_toml = r#"
            ip = '127.0.0.1'
            port = 5000
        "#
        .to_string();
        parser(&mut query_res, &test_toml)?;

        assert_eq!(
            query_res.get_metadata::<TestParse>(),
            Some(&TestParse {
                ip: String::from("127.0.0.1"),
                port: Some(5000)
            })
        );
        Ok(())
    }

    /*
    #[test]
    fn metadata_parse_build() {
        let metadata_parser = HashMap::from_iter([
            (String::from("parse_test.toml"), Box::new(parser) as MetadataParser)
        ]);
        let query = QueryBuilder::default()
            .metadata_parsers(metadata_parser)
            .build()
            .unwrap();

        // TODO make this work not only on my computer
        let path = PathBuf::from("/home/pinosaur/Temp");
        let query_res = query.scan(&path);

        assert_eq!(
            query_res.get_metadata::<TestParse>(),
            Some(&TestParse {
                ip: String::from("127.0.0.1"),
                port: Some(5000)
            })
        );
    }
    */
}
