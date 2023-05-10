//! Specify shrs completions from a derive macro
//!
//!

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Fields, Item, ItemStruct, LitStr, Meta};

/// Information on a flag
struct Flag {
    // desc: String,
    /// Long flag name
    ///
    /// Long names are passed with a double dash. For example `--verbose`.
    long: String,
    /// Short flag name
    ///
    /// Short flags can be passed with a single dash. For example `-v`.
    short: Option<char>,
}

impl Flag {
    fn new(long: impl ToString) -> Self {
        Flag {
            long: long.to_string(),
            short: None,
        }
    }
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("Could not applied on unnamed field")]
    UnamedField,
}

/// Data gathered from parsing the struct the `Completion` macro is applied to
struct DeriveContext {
    flags: Vec<Flag>,
}

impl DeriveContext {
    fn new() -> Self {
        Self { flags: vec![] }
    }
}

#[proc_macro_derive(Completion, attributes(flag))]
pub fn completion(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as Item);

    let mut ctx = DeriveContext::new();

    if let Item::Struct(item) = parsed_input {
        impl_struct(&mut ctx, item).unwrap()
    } else {
        quote! {
            compile_error!("not used on struct or enum")
        }
        .into()
    }
}

fn impl_struct(ctx: &mut DeriveContext, item: ItemStruct) -> Result<TokenStream, Error> {
    for field in item.fields {
        let field_name = field.ident.ok_or(Error::UnamedField)?;

        // check if field is marked as flag
        for attr in field.attrs.iter() {
            if attr.path().is_ident("flag") {
                // Default long flag name is name of field
                let mut flag = Flag::new(field_name.clone());

                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("long") {
                        let value = meta.value()?;
                        let s: LitStr = value.parse()?;
                        flag.long = s.value();
                    } else {
                        return Err(meta.error("unsupported attribute"));
                    }

                    Ok(())
                })
                .unwrap();
                ctx.flags.push(flag);
            }
        }
    }

    Ok(quote! {}.into())
}
