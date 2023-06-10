//! Utilities for rust based projects

use std::collections::HashMap;

use serde::Deserialize;
use shrs::anyhow;

use crate::query::{MetadataParser, Query, QueryBuilder, QueryBuilderError, QueryResult};

pub struct RustModule {
    pub cargo_toml: CargoToml,
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
pub struct CargoToml {
    package: Package,
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
pub struct Package {
    name: String,
    version: String,
    edition: String,
    description: String,
}

fn cargo_toml_parser(query_res: &mut QueryResult, content: &String) -> anyhow::Result<()> {
    let parsed: CargoToml = toml::from_str(content)?;
    query_res.add_metadata(parsed);
    Ok(())
}

pub fn module() -> Result<Query, QueryBuilderError> {
    let metadata_parser = HashMap::from_iter([(
        String::from("Cargo.toml"),
        Box::new(cargo_toml_parser) as MetadataParser,
    )]);

    QueryBuilder::default()
        .metadata_parsers(metadata_parser)
        .files(vec![String::from("Cargo.toml")])
        .build()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use shrs::anyhow;

    use super::module;
    use crate::rust::CargoToml;

    #[test]
    fn scan_rust_project() -> anyhow::Result<()> {
        let module = module()?;
        let path = std::env::current_dir()?;
        let query_res = module.scan(&path);

        let cargo_toml = query_res.get_metadata::<CargoToml>();
        println!("{:?}", cargo_toml);

        Ok(())
    }
}
