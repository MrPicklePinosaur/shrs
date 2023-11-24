use shrs::{anyhow::Result, prelude::*};

pub struct AnalyticsPlugin;

//TODO
//Builtin to retrieve analytics

//Metrics:
//command execute count
//most common directory
//Shell usage time
//Suggested aliases based off common commands
//Make stored data easily accessible to other plugins so that they can do smart things
//Maybe predict what cd is going to happen based on how often user cds from one dir to the other

//Hooks to collect analytics

impl Plugin for AnalyticsPlugin {
    fn init(&self, shell: &mut ShellConfig) -> Result<()> {
        shell.builtins.insert("analytics", AnalyticsBuiltin);
        shell.hooks.register(record_dir_change);
        shell.state.insert(AnalyticsState {});

        Ok(())
    }
}

pub struct AnalyticsBuiltin;
impl BuiltinCmd for AnalyticsBuiltin {
    fn run(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &Vec<String>,
    ) -> anyhow::Result<CmdOutput> {
        //Args, timeframe; This session or all time
        //which metric
        ctx.out.println("Analytics for ")?;
        Ok(CmdOutput::success())
    }
}
pub struct AnalyticsState {}
fn record_dir_change(
    sh: &Shell,
    ctx: &mut Context,
    rt: &mut Runtime,
    cd_ctx: &ChangeDirCtx,
) -> Result<()> {
    Ok(())
}
