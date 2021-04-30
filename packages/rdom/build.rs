use serde_derive::Deserialize;
use std::{
    collections::HashMap,
    fs::{self, read_to_string},
    path::Path,
};

use proc_macro2::TokenStream;
use quote::quote;
use sourcegen_cli::{run_sourcegen, SourceGenerator, SourcegenParameters};

// recursive function traverses `dir`
// it also adds prefixes with '-' if the file is in a folder
fn read_dir(dir: impl AsRef<Path>, prefix: String, collect: &mut Vec<(String, Generator)>) {
    for entry in fs::read_dir(dir).unwrap().map(Result::unwrap) {
        // srry, this code is kind of a mess
        //                        - maksimil
        if entry.file_type().unwrap().is_file() {
            let key = format!("{}{}", prefix, entry.file_name().to_str().unwrap());
            let key = key[0..key.len() - 5].to_string();
            let generator = Generator::load(&entry.path());
            collect.push((key, generator));
        } else if entry.file_type().unwrap().is_dir() {
            let prefix = format!("{}{}-", prefix, entry.file_name().to_str().unwrap());
            read_dir(&entry.path(), prefix, collect);
        }
    }
}

fn main() {
    // data storage
    let data = {
        let mut data = Vec::new();
        read_dir("generators", "".to_string(), &mut data);
        data
    };

    // storage for references to that data (because sourcegen is built this way)
    let dataref = data
        .iter()
        .map(|(s, g)| (s.as_str(), g as &dyn SourceGenerator))
        .collect::<Vec<_>>();

    eprintln!("{:?}", data);
    let parameters = SourcegenParameters {
        generators: &dataref[..],
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
    fn load(path: impl AsRef<Path>) -> Generator {
        eprintln!("{:?}", path.as_ref());
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
