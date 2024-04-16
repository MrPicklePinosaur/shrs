use shrs::prelude::*;

use crate::AnalyticsState;

pub struct AnalyticsBuiltin;

impl BuiltinCmd for AnalyticsBuiltin {
    fn run(
        &self,
        _sh: &Shell,
        ctx: &mut Context,
        _rt: &mut Runtime,
        _args: &[String],
    ) -> anyhow::Result<CmdOutput> {
        //Args, timeframe; This session or all time
        //which metric
        const LIMIT: usize = 5;

        ctx.state.get::<AnalyticsState>().map(|state| {
            ctx.out.println("most used commands ====").unwrap();
            let mut commands = state.commands.iter().collect::<Vec<_>>();
            commands.sort_by(|a, b| b.cmp(a));
            for (cmd, count) in commands.iter().take(LIMIT) {
                let out = format!("{cmd} {count}");
                // TODO would like ctx.out.println to be a macro like println!
                ctx.out.println(out).unwrap();
            }

            ctx.out.println("most used dirs ====").unwrap();
            let mut dirs = state.dirs.iter().collect::<Vec<_>>();
            dirs.sort_by(|a, b| b.cmp(a));
            for (dir, count) in dirs.iter().take(LIMIT) {
                let out = format!("{dir:?} {count}");
                // TODO would like ctx.out.println to be a macro like println!
                ctx.out.println(out).unwrap();
            }
        });
        Ok(CmdOutput::success())
    }
}
