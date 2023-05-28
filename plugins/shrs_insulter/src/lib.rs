use shrs::{
    anyhow,
    prelude::{AfterCommandCtx, Plugin},
    Context, Runtime, Shell,
};

pub struct InsulterPlugin;

impl Plugin for InsulterPlugin {
    fn init(&self, shell: &mut shrs::ShellConfig) {
        shell.hooks.register(insult_hook);
    }
}
fn insult_hook(
    sh: &Shell,
    sh_ctx: &mut Context,
    sh_rt: &mut Runtime,
    ctx: &AfterCommandCtx,
) -> anyhow::Result<()> {
    if (ctx.exit_code != 0) {}
    Ok(())
}
