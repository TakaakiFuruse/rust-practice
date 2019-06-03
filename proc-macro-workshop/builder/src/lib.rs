#![recursion_limit = "128"]
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_builder(&ast)
}

fn impl_builder(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        use std::error::Error;

        #[derive(Debug)]
        pub struct CommandBuilder {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }

        impl CommandBuilder {
            fn executable(&mut self, executable: String) -> &mut Self{
                self.executable = Some(executable);
                self
            }
            fn args(&mut self, args: Vec<String>) -> &mut Self{
                self.args = Some(args);
                self
            }
            fn env(&mut self, env: Vec<String>) -> &mut Self{
                self.env = Some(env);
                self
            }
            fn current_dir(&mut self, current_dir: String) -> &mut Self{
                self.current_dir = Some(current_dir);
                self
            }

            pub fn build(&self)-> Result<#name, Box<dyn Error>>{
                Ok(
                    #name {
                        executable: self.executable.clone().ok_or("NONE!!")?,
                        args: self.args.clone().ok_or("NONE!!")?,
                        env: self.env.clone().ok_or("NONE!!")?,
                        current_dir: self.current_dir.clone().ok_or("NONE!!")?,
                    }
                )
            }
        }

        impl #name {
            fn builder() -> CommandBuilder {
                CommandBuilder {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }
    };
    TokenStream::from(gen)
}
