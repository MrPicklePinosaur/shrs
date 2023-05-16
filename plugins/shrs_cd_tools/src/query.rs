//! Scan file system to match project type

#[derive(Builder)]
pub struct Query<'a> {
    /// Required files (exact match)
    files: Vec<&'a str>,
    /// Required file extensions
    extensions: Vec<&'a str>,
    /// Required directories
    dirs: Vec<&'a str>,
}

impl<'a> Query<'a> {
    /// Runs filesystem query and returns if query matched
    pub fn scan(&self) -> bool {
        todo!()
    }
}
