//! Specify shrs completions from a derive macro
//!
//!

#[macro_use]
extern crate derive_builder;

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Fields, Item, ItemStruct, LitStr, Meta};
use thiserror::__private::DisplayAsDisplay;

/// Information on the CLI itself
#[derive(Builder)]
#[builder(pattern = "mutable")]
struct Cli {
    /// Name of the command
    name: String,
}

/// Information on a flag
#[derive(Builder)]
#[builder(pattern = "mutable")]
struct Flag {
    // desc: String,
    /// Long flag name
    ///
    /// Long names are passed with a double dash. For example `--verbose`.
    long: String,
    /// Short flag name
    ///
    /// Short flags can be passed with a single dash. For example `-v`.
    #[builder(default)]
    short: Option<char>,
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("Could not applied on unnamed field")]
    UnamedField,
}

#[proc_macro_derive(Completion, attributes(flag))]
pub fn completion(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as Item);

    if let Item::Struct(item) = parsed_input {
        impl_struct(item).unwrap()
    } else {
        quote! {
            compile_error!("not used on struct or enum")
        }
        .into()
    }
}

fn impl_struct(item: ItemStruct) -> Result<TokenStream, Error> {
    let mut cli = CliBuilder::default();
    let mut flags: Vec<Flag> = vec![];

    let struct_name = &item.ident;
    cli.name(struct_name.as_display().to_string());

    for field in item.fields {
        let field_name = field.ident.ok_or(Error::UnamedField)?;

        // check if field is marked as flag
        for attr in field.attrs.iter() {
            if attr.path().is_ident("flag") {
                // Default long flag name is name of field
                let mut flag = FlagBuilder::default();
                flag.long(field_name.to_string());

                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("long") {
                        let value = meta.value()?;
                        let s: LitStr = value.parse()?;
                        flag.long(s.value());
                    } else {
                        return Err(meta.error("unsupported attribute"));
                    }

                    Ok(())
                })
                .unwrap();
                flags.push(flag.build().unwrap());
            }
        }
    }

    let cli = cli.build().unwrap();

    let flag_rules = flag_rules(cli, flags).unwrap();

    let output = quote! {
        // TODO might run into issues using a use statement here
        use shrs::line::completion::{DefaultCompleter, Rule, Pred, Action};

        impl #struct_name {
            pub fn rules(&self, comp: &mut DefaultCompleter) -> Vec<Rule> {

                // Rules for flags
                // comp.register(#flag_rules)

                todo!()
            }
        }
    }
    .into();
    Ok(output)
}

/// Generate rules based off of flags that were parsed
fn flag_rules(cli: Cli, flags: Vec<Flag>) -> Result<TokenStream, Error> {
    use shrs::line::completion::{
        cmdname_eq_pred, default_format, flag_pred, Action, Completion, CompletionCtx,
        DefaultCompleter, Pred, Rule,
    };

    let cli_name = &cli.name;
    let long_flags = flags
        .iter()
        .map(|f| {
            let f = format!("--{}", f.long);
            quote! { f }
        })
        .collect::<Vec<_>>();

    let output = quote! {
        {
            let long_flags_action = |ctx: &CompletionCtx| -> Vec<Completion> {
                default_format(
                    vec![
                        #(#long_flags)*
                    ]
                )
            };
            Rule::new(
                Pred::new(cmdname_eq_pred(cli_name.into())).and(flag_pred),
                Box::new(long_flags_action)
            );
        }
    }
    .into();
    Ok(output)
}
