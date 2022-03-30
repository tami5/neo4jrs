mod bolt_struct;
mod label;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(BoltStruct, attributes(signature))]
pub fn derive_boltstruct(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    bolt_struct::expand(ast).unwrap_or_else(to_stream).into()
}

#[proc_macro_derive(Label)]
pub fn derive_label(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    label::expand(ast).unwrap_or_else(to_stream).into()
}

fn to_stream(errors: Vec<syn::Error>) -> proc_macro2::TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    quote!(#(#compile_errors)*)
}
