//! More completions for shrs
//!
//!

use std::{fs, path::PathBuf, rc::Rc};

use dirs::home_dir;
use rhai::{Array, Engine, Scope};
use setup::setup_engine;
use shrs::prelude::*;

pub struct CompletionsPlugin;
pub mod setup;

/// retrieve completions folder, create it if it doesnt exist and copy over completions
fn completions_folder() -> PathBuf {
    let mut folder = home_dir().unwrap();
    folder.push(".config/shrs/completions");

    // Create the folder if it doesn't exist
    if !folder.exists() {
        fs::create_dir_all(&folder).unwrap();
    }
    folder
}

impl Plugin for CompletionsPlugin {
    fn init(&self, _: &mut ShellConfig) -> anyhow::Result<()> {
        Ok(())
    }
    fn post_init(&self, _sh: &Shell, ctx: &mut Context, _rt: &mut Runtime) -> ::anyhow::Result<()> {
        let mut e = Engine::new();
        setup_engine(&mut e);
        let engine = Rc::new(e);
        let folder = completions_folder();

        for p in fs::read_dir(folder).unwrap() {
            let path = p.unwrap().path();
            let pred_path = path.clone();
            let comp_path = path.clone();

            let compiled = engine.compile_file(path);
            let ast = Rc::new(match compiled {
                Ok(ast) => ast,
                Err(e) => {
                    eprintln!("Rhai script compile error {}", e);
                    continue;
                },
            });
            let ast_ref_comp = ast.clone();

            let engine_ref_pred = engine.clone();
            let engine_ref_comp = engine.clone();

            ctx.completer.register(Rule::new(
                Pred::new(move |c| {
                    let mut scope = Scope::new();

                    let predicate = engine_ref_pred.call_fn::<bool>(
                        &mut scope,
                        &ast,
                        "predicate",
                        (c.clone(),),
                    );
                    match predicate {
                        Ok(p) => p,
                        Err(_) => {
                            eprintln!("predicate in {:?} failed", pred_path.clone());
                            false
                        },
                    }
                }),
                move |c| -> Vec<Completion> {
                    let mut scope = Scope::new();

                    let completions = engine_ref_comp.call_fn::<Array>(
                        &mut scope,
                        &ast_ref_comp,
                        "completions",
                        (c.clone(),),
                    );
                    match completions {
                        Ok(c) => c.iter().map(|x| x.clone().cast::<Completion>()).collect(),
                        Err(_) => {
                            completions.unwrap();
                            eprintln!("completion in {:?} failed", comp_path);
                            vec![]
                        },
                    }
                },
            ));
        }

        Ok(())
    }
    fn meta(&self) -> PluginMeta {
        PluginMeta::new("Completions", "Provides Rhai completions for shrs", None)
    }
}
