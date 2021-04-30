use std::{fs::read_to_string, path::Path};

use proc_macro2::TokenStream;
use quote::quote;
use sourcegen_cli::SourceGenerator;

use crate::template::Template;

#[derive(Debug)]
pub(crate) struct Generator(pub Template);

impl Generator {
    pub(crate) fn load(path: impl AsRef<Path>) -> Generator {
        let contents = read_to_string(path).unwrap();
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
            let fields =
                template
                    .fields
                    .0
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
