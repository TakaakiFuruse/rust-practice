extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(builder)]
pub fn derive_builder(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = &ast.ident;
    let bname = format!("{}Builder", &name);
    let builder_name = syn::Ident::new(&bname, name.span());
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        unimplemented!();
    };
    let accessors = fields.iter().filter(|elm| elm.ident.is_some()).map(|elm| {
        let attr_name = elm.to_owned().ident.unwrap();
        let with_attr_name = format!("with_{}", attr_name);
        let method_name = syn::Ident::new(&with_attr_name, attr_name.span());
        let method = quote! {
                pub fn #method_name(mut self, var: String) -> Self {
                    self.#attr_name = var;
                    self
                }
        };
        method
    });
    let struct_fields = fields.iter().filter(|elm| elm.ident.is_some()).map(|elm| {
        let attr_name = elm.to_owned().ident.unwrap();
        let method = quote! {
            pub #attr_name: String
        };
        method
    });
    let build_fields = fields.iter().filter(|elm| elm.ident.is_some()).map(|elm| {
        let attr_name = elm.to_owned().ident.unwrap();
        let method = quote! {
            #attr_name: self.#attr_name.to_owned()
        };
        method
    });
    let new_fields = fields.iter().filter(|elm| elm.ident.is_some()).map(|elm| {
        let attr_name = elm.to_owned().ident.unwrap();
        let method = quote! {
            #attr_name: "None".to_string()
        };
        method
    });
    let impls = quote! {
        pub struct #builder_name {
            #(#struct_fields,)*
        }
        impl #builder_name {
            #(#accessors)*
            pub fn build(&self) -> #name {
                #name{
                    #(#build_fields,)*
                }
            }
            pub fn new() -> #builder_name {
                #builder_name{
                    #(#new_fields,)*
                }
            }
        }
    };
    impls.into()
}
