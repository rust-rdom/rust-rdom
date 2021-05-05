use std::str::FromStr;

use anyhow::{anyhow, Context};
use proc_macro2::TokenStream;
use quote::quote;
use sourcegen_cli::tokens::NewLine;
use sourcegen_cli::SourceGenerator;
use syn::{Lit, Meta, MetaNameValue, NestedMeta};

#[derive(Debug)]
struct BehaviorScript(Vec<BehaviorStatement>);

#[derive(Debug)]
struct BehaviorStatement {
    behavior: String,
    args: Vec<String>,
}

impl FromStr for BehaviorScript {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut output = Vec::new();

        for s in s.split_terminator(';') {
            let s = s.parse()?;
            output.push(s);
        }

        Ok(BehaviorScript(output))
    }
}

impl FromStr for BehaviorStatement {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split_whitespace().map(str::to_string);

        let behavior = s
            .next()
            .with_context(|| anyhow!("No script name provided"))?;
        let args = s.collect();

        Ok(BehaviorStatement { behavior, args })
    }
}

pub struct BehaviorGenerator;

macro_rules! match_behaviors {
    ($s:ident / $($lit:literal -> $fun:ident),*) => {
        {
            match $s.as_str() {
                $(
                    $lit => Ok($fun as Builder),
                )*
                _ => Err(anyhow!("script was not found")),
            }
        }
    };
}

type Builder = fn(&syn::ItemStruct, Vec<String>) -> Result<TokenStream, anyhow::Error>;

impl SourceGenerator for BehaviorGenerator {
    fn generate_struct(
        &self,
        args: syn::AttributeArgs,
        item: &syn::ItemStruct,
    ) -> Result<Option<TokenStream>, anyhow::Error> {
        let script: BehaviorScript = args
            .into_iter()
            .find_map(|e| match e {
                NestedMeta::Meta(Meta::NameValue(MetaNameValue { path, lit, .. })) => {
                    if path.get_ident()?.to_string() == "script" {
                        match lit {
                            Lit::Str(s) => Some(s.value()),
                            _ => None,
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .with_context(|| anyhow!("This is not a valid script"))?
            .parse()?;

        let mut output = TokenStream::new();

        for BehaviorStatement { behavior, args } in script.0 {
            let gen = match_behaviors! {
                behavior /
                "SandboxMember" -> sandbox_member
            }?(item, args)?;

            output.extend(quote! {
                #NewLine
                #gen
            });
        }

        Ok(Some(quote! {
            #item
            #NewLine
            #output
        }))
    }
}

fn sandbox_member(item: &syn::ItemStruct, args: Vec<String>) -> Result<TokenStream, anyhow::Error> {
    let ident = &item.ident;

    let mut args = args.into_iter();

    let field: TokenStream = match (args.next(), args.next()) {
        (Some(field), None) => Ok(field),
        _ => Err(anyhow!(
            "Correct usage for SandboxMember: SandboxMember context"
        )),
    }?
    .parse()
    .unwrap();

    Ok(quote! {
        #[sourcegen::generated]
        impl #ident {
            /// gets `Weak<Sandbox>` to the `Sandbox` that it is in
            pub fn get_context(&self) -> Weak<Sandbox> {
                self.#field.clone()
            }
        }
        #NewLine
        #[sourcegen::generated]
        impl SandboxMemberBehavior for #ident {
            fn get_context(&self) -> Weak<Sandbox> {
                self.get_context()
            }
        }
    })
}
