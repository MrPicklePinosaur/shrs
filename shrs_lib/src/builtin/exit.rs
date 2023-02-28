use super::BuiltinCmd;

#[derive(Default)]
pub struct ExitBuiltin {}

impl BuiltinCmd for ExitBuiltin {
    fn run(
        &self,
        ctx: &mut crate::shell::Context,
        args: &Vec<String>,
    ) -> anyhow::Result<std::process::Child> {
        std::process::exit(0)
    }
}
