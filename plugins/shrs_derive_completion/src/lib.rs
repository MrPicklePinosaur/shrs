//! Specify shrs completions from a derive macro
//!
//!

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Fields, Item, ItemStruct};

#[proc_macro_derive(Completion, attributes(flag))]
pub fn completion(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as Item);

    if let Item::Struct(item) = parsed_input {
        impl_struct(item)
    } else {
        quote! {
            compile_error!("not used on struct or enum")
        }
        .into()
    }
}

fn impl_struct(item: ItemStruct) -> TokenStream {
    quote! {}.into()
}
