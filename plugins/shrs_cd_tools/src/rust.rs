//! Utilities for rust based projects

use crate::query::{Query, QueryBuilder, QueryBuilderError};

pub struct RustModule {
    // pub cargo_toml:
}

pub fn module() -> Result<Query, QueryBuilderError> {
    QueryBuilder::default()
        .files(vec![String::from("Cargo.toml")])
        .build()
}
