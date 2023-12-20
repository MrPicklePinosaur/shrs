use shrs::{anyhow::Result, prelude::*};

use crate::AnalyticsState;

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
        ctx.state.get::<AnalyticsState>().map(|state| {
            let mut commands = state.commands.iter().collect::<Vec<_>>();
            commands.sort_by(|a, b| b.cmp(a));
            for (cmd, count) in commands.iter() {
                let out = format!("{cmd} {count}");
                // TODO would like ctx.out.println to be a macro like println!
                ctx.out.println(out).unwrap();
            }
        });
        Ok(CmdOutput::success())
    }
}
