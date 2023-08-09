//! Utilities for nodejs based projects

use std::collections::HashMap;

use serde::Deserialize;
use shrs::anyhow;

use crate::query::{MetadataParser, Query, QueryBuilder, QueryBuilderError, QueryResult};

#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
pub struct PackageJson {
    pub name: String,
    pub version: String,
    pub description: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
pub struct NodeJs {
    /// Version of node js
    pub version: String,
}

fn package_json_parser(query_res: &mut QueryResult, content: &String) -> anyhow::Result<()> {
    Ok(())
}

pub fn module() -> Result<Query, QueryBuilderError> {
    let metadata_parser = HashMap::from_iter([(
        String::from("package.json"),
        Box::new(package_json_parser) as MetadataParser,
    )]);

    QueryBuilder::default()
        .metadata_parsers(metadata_parser)
        .files(vec![String::from("package.json")])
        .build()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use shrs::anyhow;

    use super::module;
    use crate::node::PackageJson;

    #[test]
    fn scan_node_project() -> anyhow::Result<()> {
        let module = module()?;
        let path = std::env::current_dir()?;
        let query_res = module.scan(&path);

        let package_json = query_res.get_metadata::<PackageJson>();
        println!("{:?}", package_json);

        Ok(())
    }
}
