//! More completions for shrs
//!
//!

pub mod completions;
mod helpers;
use std::{fs, rc::Rc};

use ::anyhow::anyhow;
use dirs::home_dir;
use rhai::{Array, Engine, Scope};
use setup::setup_engine;
use shrs::prelude::*;

pub struct CompletionsPlugin;
pub mod setup;

impl Plugin for CompletionsPlugin {
    fn init(&self, _: &mut ShellConfig) -> anyhow::Result<()> {
        Ok(())
    }
    fn post_init(&self, sh: &Shell, ctx: &mut Context, rt: &mut Runtime) -> ::anyhow::Result<()> {
        let mut e = Engine::new();
        setup_engine(&mut e);
        let engine = Rc::new(e);
        let mut folder = home_dir().unwrap();
        folder.push(".config/shrs/completions");
        for p in fs::read_dir(folder).unwrap() {
            let path = p.unwrap().path();
            let compiled = engine.compile_file(path);
            let ast = Rc::new(match compiled {
                Ok(ast) => ast,
                Err(e) => {
                    eprintln!("Rhai script compile error {}", e);
                    return Err(anyhow!("Can't compile"));
                },
            });
            let ast1 = ast.clone();

            let e1 = engine.clone();
            let e2 = engine.clone();

            ctx.completer.register(Rule::new(
                Pred::new(move |c| {
                    let mut scope = Scope::new();

                    let predicate: bool = e1
                        .call_fn::<bool>(&mut scope, &ast, "predicate", (c.clone(),))
                        .unwrap();
                    predicate
                }),
                move |c| -> Vec<Completion> {
                    let mut scope = Scope::new();

                    let completions: Vec<Completion> = e2
                        .call_fn::<Array>(&mut scope, &ast1, "completions", (c.clone(),))
                        .unwrap()
                        .iter()
                        .map(|x| x.clone().cast::<Completion>())
                        .collect();
                    completions
                },
            ));
        }

        Ok(())
    }
}
