use serde_derive::Deserialize;
use std::{collections::HashMap, fs::read_to_string};

use proc_macro2::TokenStream;
use quote::quote;
use sourcegen_cli::{run_sourcegen, SourceGenerator, SourcegenParameters};

macro_rules! declare_generators {
    ($($g:literal),*) => {
        &[
            $(
                ($g, &Generator::load($g) as &dyn SourceGenerator),
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

#[derive(Deserialize, Debug)]
struct Template {
    fields: HashMap<String, String>,
}

#[derive(Debug)]
struct Generator(pub Template);

impl Generator {
    fn load(path: &str) -> Generator {
        let contents = read_to_string(format!("generators/{}.yaml", path)).unwrap();
        Generator(serde_yaml::from_str(&contents).unwrap())
    }
}

impl SourceGenerator for Generator {
    fn generate_struct(
        &self,
        _args: syn::AttributeArgs,
        item: &syn::ItemStruct,
    ) -> Result<Option<TokenStream>, anyhow::Error> {
        let template = &self.0;

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

        let definition = {
            let fields = template
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
