//! Specify shrs completions from a derive macro
//!
//!

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Fields, Item, ItemStruct};

#[proc_macro_derive(Builder)]
pub fn builder(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as Item);

    if let Item::Struct(item) = parsed_input {
        impl_deref(item)
    } else {
        quote! {
            compile_error!("not used on struct")
        }
        .into()
    }
}

fn impl_deref(item: ItemStruct) -> TokenStream {
    todo!()
}
