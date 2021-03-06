extern crate proc_macro;
extern crate toml;

use proc_macro::TokenStream;
use quote::quote;
use std::fs;
use std::io::{BufReader, Read};
use toml::Value;

use syn::{parse_macro_input, DeriveInput};

fn file_open(path: String) -> Result<String, String> {
    let mut file_content = String::new();

    let mut fr = fs::File::open(path)
        .map(|f| BufReader::new(f))
        .map_err(|e| e.to_string())?;

    fr.read_to_string(&mut file_content)
        .map_err(|e| e.to_string())?;

    Ok(file_content)
}

#[proc_macro_derive(order)]
pub fn order_builder(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = &ast.ident;
    let enum_variants = if let syn::Data::Enum(syn::DataEnum { variants, .. }) = ast.data {
        variants
    } else {
        unimplemented!();
    };
    let enum_len = &enum_variants.len();

    let order_config: toml::Value = file_open("src/config.toml".to_string())
        .unwrap()
        .parse::<Value>()
        .unwrap();

    let orders: Vec<&str> = order_config["order"].as_str().unwrap().split(",").collect();

    assert_eq!(&orders.len(), enum_len);

    let enum_fields = orders.iter().enumerate().map(|(i, elm)| {
        let elm_ident = syn::Ident::new(&elm, name.span());
        let arm = quote! {
            #name::#elm_ident => #i as i32,
        };
        arm
    });

    let order_func = quote! {
        impl #name {
            pub fn order(&self) -> i32{
                match self {
                    #(#enum_fields)*
                }
            }
        }
    };
    order_func.into()
}
