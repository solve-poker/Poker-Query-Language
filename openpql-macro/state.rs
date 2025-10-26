use std::{
    collections::HashSet,
    sync::{Mutex, OnceLock},
};

use quote::quote;

use super::FnMetadata;

static ARMS: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
static IMPLEMENTED_TYPES: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

pub fn arms() -> &'static Mutex<Vec<String>> {
    ARMS.get_or_init(|| Mutex::new(vec![]))
}

pub fn implemented_types() -> &'static Mutex<HashSet<String>> {
    IMPLEMENTED_TYPES.get_or_init(|| Mutex::new(HashSet::new()))
}

pub fn is_already_implemented(ty: &syn::TypeBareFn) -> bool {
    implemented_types()
        .lock()
        .unwrap()
        .contains(&FnMetadata::to_hash_key(ty))
}

pub fn mark_as_implemented(ty: &syn::TypeBareFn) {
    implemented_types()
        .lock()
        .unwrap()
        .insert(FnMetadata::to_hash_key(ty));
}

pub fn register_match_arm(
    fnname: &syn::Ident,
    ty: &syn::TypeBareFn,
    alias: Option<&str>,
) {
    let str_fnname = fnname.to_string().replace('_', "");
    let match_arm = quote! {
        #str_fnname => Ok(&(#fnname as #ty))
    };
    arms().lock().unwrap().push(match_arm.to_string());

    if let Some(alias_name) = alias {
        let match_arm = quote! {
            #alias_name => Ok(&(#fnname as #ty))
        };
        arms().lock().unwrap().push(match_arm.to_string());
    }
}
