use std::{fs::read_to_string, path::Path};

use crate::behavior::BEHAVIORS;
use proc_macro2::TokenStream;
use quote::quote;
use sourcegen_cli::tokens::NewLine;
use sourcegen_cli::SourceGenerator;

use crate::template::Template;

#[derive(Debug)]
pub struct Generator(pub Template);

impl Generator {
    pub fn load(path: impl AsRef<Path>) -> Generator {
        let contents = read_to_string(path).expect("Could not read to string");
        Generator(toml::from_str(&contents).expect("Could not parse TOML"))
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

        let mut output = {
            let fields = template
                .fields
                .0
                .iter()
                .fold(TokenStream::new(), |mut acc, data| {
                    acc.extend(data.ts.clone());
                    acc
                });

            let doc: &str = &template.doc;

            quote! {
                #[doc = #doc]
                #attrs
                #vis struct #ident #generics {
                    #fields
                }
            }
        };

        if let Some(weak_name) = template.weak_name.as_ref() {
            let ident: TokenStream = weak_name.parse().unwrap();
            let fields = template
                .fields
                .0
                .iter()
                .fold(TokenStream::new(), |mut acc, data| {
                    acc.extend(data.wts.clone());
                    acc
                });

            let doc: &str = template.weak_doc.as_ref().unwrap_or(&template.doc);

            output.extend(quote! {
                #[doc = #doc]
                #[sourcegen::generated]
                #attrs
                #vis struct #ident #generics {
                    #fields
                }
            })
        }

        for behavior in template.behaviors.iter() {
            let (base, rest) = {
                let mut split = behavior.split_whitespace();
                let base = split.next().unwrap();
                let rest = split.collect::<Vec<_>>();
                (base, rest)
            };

            let implementor = BEHAVIORS.get(base).unwrap();
            let implementation = implementor(rest, item)?;

            output.extend(quote! {
                #NewLine
                #implementation
            });
        }

        Ok(Some(output))
    }
}
