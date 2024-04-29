extern crate proc_macro;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HookCtx)]
pub fn derive_ctx(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the type for which we are deriving the trait
    let type_name = &input.ident;

    // Generate the implementation of the Ctx trait for the specified type
    let expanded = quote! {
        impl HookCtx for #type_name {}
    };

    // Return the generated implementation as a TokenStream
    proc_macro::TokenStream::from(expanded)
}
