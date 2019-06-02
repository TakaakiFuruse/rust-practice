extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_builder(&ast)
}

fn impl_builder(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl #name {
            fn builder() {
                println!("I am builder func !!");
            }
        }
    };
    gen.into()
}
