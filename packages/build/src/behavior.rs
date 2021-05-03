use anyhow::anyhow;
use lazy_static::lazy_static;
use proc_macro2::TokenStream;
use quote::quote;
use sourcegen_cli::tokens::PlainComment;
use std::collections::HashMap;
use syn::parse::Parse;

pub(crate) type BehaviorTemplate =
    fn(Vec<&str>, &syn::ItemStruct) -> Result<TokenStream, anyhow::Error>;
pub(crate) type BehaviorStore = HashMap<&'static str, BehaviorTemplate>;

lazy_static! {
    pub static ref BEHAVIORS: BehaviorStore = get_behavior_store();
}

pub(crate) fn get_behavior_store() -> BehaviorStore {
    let mut store = HashMap::new();

    // SandboxMember
    store.insert("SandboxMember", sandbox_member as BehaviorTemplate);

    store
}

// implementation for SandboxMemberBehavior
// SandboxMember(context)
fn sandbox_member(fields: Vec<&str>, item: &syn::ItemStruct) -> Result<TokenStream, anyhow::Error> {
    if fields.len() != 1 {
        return Err(anyhow!("Wrong number of arguments, expected 1"));
    }

    let field: TokenStream = fields[0].parse().unwrap();

    let ident = &item.ident;
    let generics = &item.generics;

    Ok(quote! {
        #[sourcegen::generated]
        impl #generics #ident #generics {
            #[doc = " gets `Weak<Sandbox>` to the `Sandbox` that it is in"]
            pub fn get_context(&self) -> Weak<Sandbox> {
                self.#field.clone()
            }
        }

        #[sourcegen::generated]
        impl #generics SandboxMemberBehavior for #ident #generics {
            fn get_context(&self) -> Weak<Sandbox> {
                self.get_context()
            }
        }
    })
}
