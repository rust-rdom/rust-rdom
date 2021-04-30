use serde_derive::Deserialize;
use std::{
    collections::HashMap,
    fmt::format,
    fs::{self, read_to_string},
};

use proc_macro2::TokenStream;
use quote::quote;
use sourcegen_cli::{run_sourcegen, SourceGenerator, SourcegenParameters};

macro_rules! declare_generators {
    ($($g:literal),*) => {
        &[
            $(
                ($g, &Generator($g) as &dyn SourceGenerator),
            )*
        ]
    };
}

fn main() {
    let parameters = SourcegenParameters {
        generators: declare_generators!("window"),
        ..Default::default()
    };
    run_sourcegen(&parameters).unwrap();
}

fn load_yaml(name: &str) -> String {
    read_to_string(format!("generators/{}.yaml", name)).unwrap()
}

#[derive(Deserialize)]
struct Template {
    fields: HashMap<String, String>,
}

#[derive(Debug)]
struct Generator<'s>(pub &'s str);

impl SourceGenerator for Generator<'_> {
    fn generate_struct(
        &self,
        _args: syn::AttributeArgs,
        item: &syn::ItemStruct,
    ) -> Result<Option<TokenStream>, anyhow::Error> {
        let ident = &item.ident;
        let vis = &item.vis;

        let attrs = item
            .attrs
            .clone()
            .into_iter()
            .fold(TokenStream::new(), |mut acc, attr| {
                acc.extend(quote!(#attr));
                acc
            });

        let generics = &item.generics;

        let generator: Template = serde_yaml::from_str(&load_yaml(self.0)).unwrap();

        let definition = {
            let fields =
                generator
                    .fields
                    .iter()
                    .fold(TokenStream::new(), |mut acc, (ident, ty)| {
                        let stream: TokenStream = format!("{}: {},", ident, ty).parse().unwrap();
                        acc.extend(stream);
                        acc
                    });

            quote! {
                #attrs
                #vis struct #ident #generics {
                    #fields
                }
            }
        };

        Ok(Some(definition))
    }
}
