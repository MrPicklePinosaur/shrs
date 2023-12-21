use shrs::{anyhow::Result, prelude::*};

use crate::AnalyticsPlugin;

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
        const LIMIT: usize = 5;

        ctx.state.get::<AnalyticsPlugin>().map(|state| {
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
