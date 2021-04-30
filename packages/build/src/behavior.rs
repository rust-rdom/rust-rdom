use anyhow::anyhow;
use lazy_static::lazy_static;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

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

    let field = quote::format_ident!("{}", fields[0]);

    let ident = &item.ident;
    let generics = &item.generics;

    Ok(quote! {
        impl #generics SandboxMemberBehavior for #ident #generics {
            fn get_context(&self) -> Weak<Sandbox> {
                self.#field.clone()
            }
        }
    })
}
