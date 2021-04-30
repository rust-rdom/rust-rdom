use proc_macro2::TokenStream;
use quote::quote;
use sourcegen_cli::tokens::{NewLine, PlainComment};
use sourcegen_cli::{run_sourcegen, SourceGenerator, SourcegenParameters};

pub fn main() {
    let parameters = SourcegenParameters {
        generators: &[("sandbox-member", &SandboxMember)],
        ..Default::default()
    };
    run_sourcegen(&parameters).unwrap();
}

struct SandboxMember;

impl SourceGenerator for SandboxMember {
    fn generate_struct(
        &self,
        _args: syn::AttributeArgs,
        item: &syn::ItemStruct,
    ) -> Result<Option<TokenStream>, anyhow::Error> {
        let vis = &item.vis;
        let ident = &item.ident;
        let generics = &item.generics;

        let fields = {
            let mut fields = quote! {
                #PlainComment "generated"
                context: Weak<Sandbox>,
                #NewLine
            };

            for field in &item.fields {
                if field.ident.clone().unwrap().to_string() == "context" {
                    return Ok(Some(quote!(#item)));
                }

                fields.extend(quote!(#field,));
            }

            fields
        };

        let head = quote! {
            #vis struct #ident #generics {
                #fields
            }
        };

        let implementation = quote! {
            #PlainComment "generated"
            impl #generics SandboxMemberBehavior for #ident #generics {
                fn get_context(&self) -> Weak<Sandbox> {
                    self.context.clone()
                }
            }
        };

        Ok(Some(quote! {
            #head
            #NewLine
            #implementation
        }))
    }
}
