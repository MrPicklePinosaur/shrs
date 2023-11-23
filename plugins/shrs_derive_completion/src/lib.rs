//! Specify shrs completions from a derive macro
//!
//!

#[macro_use]
extern crate derive_builder;

extern crate proc_macro;

use quote::quote;
use syn::{parse_macro_input, Item, ItemStruct, LitStr};

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
    UnnamedField,
}

#[proc_macro_derive(Completion, attributes(flag))]
pub fn completion(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as Item);

    if let Item::Struct(item) = parsed_input {
        impl_struct(item).unwrap().into()
    } else {
        quote! {
            compile_error!("not used on struct or enum")
        }
        .into()
    }
}

fn impl_struct(item: ItemStruct) -> Result<proc_macro2::TokenStream, Error> {
    let mut cli = CliBuilder::default();
    let mut flags: Vec<Flag> = vec![];

    let struct_name = &item.ident;
    cli.name(struct_name.to_string().to_ascii_lowercase());

    for field in item.fields.iter() {
        let field_name = field.ident.clone().ok_or(Error::UnnamedField)?;

        // check if field is marked as flag
        for attr in field.attrs.iter() {
            if attr.path().is_ident("flag") {
                // Default long flag name is name of field
                let mut flag = FlagBuilder::default();
                flag.long(field_name.to_string());

                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("long") {
                        let value = meta.value()?;
                        let s = value.parse::<LitStr>()?.value();
                        flag.long(s);
                    } else if meta.path.is_ident("short") {
                        let c = if let Ok(value) = meta.value() {
                            value.parse::<LitStr>()?.value()
                        } else {
                            // if no specific short flag is passed, use the first character of the
                            // current field
                            field_name.to_string()
                        };
                        let c = c
                            .chars()
                            .next()
                            .ok_or(meta.error("expected short flag with single character"))?;
                        flag.short(Some(c));
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

    let _flag_rules = flag_rules(cli, flags).unwrap();

    let output = quote! {
        // TODO might run into issues using a use statement here
        use shrs::line::completion::{
            cmdname_eq_pred, default_format, flag_pred, Action, Completion, CompletionCtx,
            DefaultCompleter, Pred, Rule,
        };

        impl #struct_name {
            pub fn rules(comp: &mut DefaultCompleter) {

                // Rules for flags
                comp.register(#_flag_rules);
            }
        }
    };
    Ok(output)
}

/// Generate rules based off of flags that were parsed
fn flag_rules(cli: Cli, flags: Vec<Flag>) -> Result<proc_macro2::TokenStream, Error> {
    let cli_name = &cli.name;
    let long_flags = flags
        .iter()
        .map(|f| {
            let f = format!("--{}", f.long);
            quote! { #f.into() }
        })
        .collect::<Vec<_>>();

    let short_flags = flags
        .iter()
        .filter_map(|f| {
            f.short.map(|f| {
                let f = format!("-{f}");
                quote! { #f.into() }
            })
        })
        .collect::<Vec<_>>();

    let output = quote! {
        {
            let flags_action = |ctx: &CompletionCtx| -> Vec<Completion> {
                default_format(
                    vec![
                        #(#short_flags),*
                        ,
                        #(#long_flags),*
                    ]
                )
            };
            Rule::new(
                Pred::new(cmdname_eq_pred(#cli_name.into())).and(flag_pred),
                Box::new(flags_action)
            )
        }
    };
    Ok(output)
}
