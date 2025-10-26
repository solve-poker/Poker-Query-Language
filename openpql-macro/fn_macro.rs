#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::missing_panics_doc)]
#![cfg_attr(coverage_nightly, coverage(off))]

mod codegen;
mod meta_arg;
mod meta_attr;
mod meta_fn;
mod meta_rtn;
mod state;

use codegen::generate_pqlfn_impl;
use meta_arg::ArgMetadata;
use meta_attr::AttrItems;
use meta_fn::FnMetadata;
use meta_rtn::RtnMetadata;
use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use state::{arms, register_match_arm};
use syn::{
    FnArg, GenericArgument, Ident, ItemFn, LitStr, PathArguments, ReturnType,
    Token, Type, TypeBareFn, TypePath,
    parse::{Parse, ParseStream, Result},
    parse_quote,
    punctuated::Punctuated,
};

type TokenStream2 = proc_macro2::TokenStream;

#[proc_macro_attribute]
pub fn pqlfn(attr: TokenStream, item: TokenStream) -> TokenStream {
    let metadata = FnMetadata::new(&attr, &item);

    if !metadata.no_parse {
        register_match_arm(
            &metadata.name,
            &metadata.fn_type,
            metadata.alias.as_deref(),
        );
    }

    let fndef = TokenStream2::from(item);
    let impl_pqlfn = generate_pqlfn_impl(&metadata);

    let output = quote! {
        #fndef
        #impl_pqlfn
    };

    output.into()
}

#[proc_macro]
pub fn pqlfn_fromstr(item: TokenStream) -> TokenStream {
    let arms: Vec<TokenStream2> = arms()
        .lock()
        .unwrap()
        .iter()
        .filter_map(|ln| ln.parse().ok())
        .collect();

    let err: TokenStream2 = item.into();

    quote! {
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_ascii_lowercase().as_str() {
                #(#arms,)*
                _ => #err,
            }
        }
    }
    .into()
}

fn typepath_to_id(ty_path: &TypePath) -> Ident {
    ty_path
        .path
        .segments
        .last()
        .unwrap_or_else(|| {
            panic!("{} not supported", ty_path.to_token_stream())
        })
        .ident
        .clone()
}

fn type_to_id(ty: &Type) -> Ident {
    fn inner(ty: &Type) -> Option<Ident> {
        if let Type::Path(TypePath { path, .. }) = ty {
            path.segments.last().map(|seg| seg.ident.clone())
        } else {
            None
        }
    }

    inner(ty)
        .unwrap_or_else(|| panic!("{} not supported", ty.to_token_stream()))
}

fn id_to_pql_type(id: &Ident) -> TokenStream2 {
    let str_inner = id.to_string();
    match str_inner.as_str() {
        "PQLPlayer" => quote! { PQLType::PLAYER },
        "PQLStreet" => quote! { PQLType::STREET },
        "PQLEquity" => quote! { PQLType::EQUITY },
        "PQLNumeric" => quote! { PQLType::NUMERIC },
        "PQLCardCount" => quote! { PQLType::CARDCOUNT },
        "PQLLong" => quote! { PQLType::LONG },
        "PQLDouble" => quote! { PQLType::DOUBLE },
        "PQLCard" => quote! { PQLType::CARD },
        "PQLBoolean" => quote! { PQLType::BOOLEAN },
        "PQLRankSet" => quote! { PQLType::RANKSET },
        "PQLRank" => quote! { PQLType::RANK },
        "PQLFlopHandCategory" => quote! { PQLType::FLOPHANDCATEGORY },
        "PQLHiRating" => quote! { PQLType::HIRATING },
        "PQLHandType" => quote! { PQLType::HANDTYPE },
        "PQLFraction" => quote! { PQLType::FRACTION },
        "PQLString" => quote! { PQLType::STRING },
        "PQLRange" => quote! { PQLType::RANGE },
        "PQLBoardRange" => quote! { PQLType::BOARDRANGE },

        _ => panic!("Unsupported type: {str_inner}"),
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Unsupported type")]
    fn test_unknown_type() {
        let id: Ident = parse_quote!(Unknown);
        id_to_pql_type(&id);
    }
}
