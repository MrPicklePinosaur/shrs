#[derive(Debug)]
pub struct PluginMeta {
    pub name: String,
    pub description: String,
}
impl Default for PluginMeta {
    fn default() -> Self {
        Self {
            name: String::from("unnamed plugin"),
            description: String::from("a plugin for shrs"),
        }
    }
}
