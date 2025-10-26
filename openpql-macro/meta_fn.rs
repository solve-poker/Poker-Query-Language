#![allow(clippy::wildcard_imports)]

use super::*;

pub struct FnMetadata {
    pub name: Ident,
    pub fn_type: TypeBareFn,
    pub args: Vec<ArgMetadata>,
    pub rtn: RtnMetadata,
    pub no_parse: bool,
    pub alias: Option<String>,
}

impl FnMetadata {
    pub fn new(attr: &TokenStream, item: &TokenStream) -> Self {
        let function: ItemFn =
            syn::parse(item.clone()).expect("Failed to parse function");
        let args = function.sig.inputs.iter().map(ArgMetadata::new).collect();
        let name = function.sig.ident.clone();
        let fn_type = Self::to_fn_type(&function);
        let rtn = RtnMetadata::new(&function.sig.output);

        let attributes: AttrItems =
            syn::parse(attr.clone()).expect("Failed to parse attributes");
        let (no_parse, alias) = attributes.into_meta();

        Self {
            name,
            fn_type,
            args,
            rtn,
            no_parse,
            alias,
        }
    }

    fn to_fn_type(item: &ItemFn) -> TypeBareFn {
        let input_types = item.sig.inputs.iter().map(|arg| match arg {
            syn::FnArg::Typed(pat_type) => &pat_type.ty,
            syn::FnArg::Receiver(_) => panic!("self not supported"),
        });
        let output = &item.sig.output;

        parse_quote! { fn(#(#input_types),*) #output }
    }

    pub fn to_hash_key(ty: &TypeBareFn) -> String {
        ty.to_token_stream().to_string()
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    fn test_fn_type() {
        let item1: ItemFn = parse_quote! {
            fn test(arg0: usize, arg1: bool) -> bool {}
        };

        let item2: ItemFn = parse_quote! {
            fn _test(
                _arg0: usize,
                _arg1: bool
            ) -> bool {}
        };

        let expected: TypeBareFn = parse_quote!(fn(usize, bool) -> bool);

        let key1 = FnMetadata::to_hash_key(&FnMetadata::to_fn_type(&item1));
        let key2 = FnMetadata::to_hash_key(&FnMetadata::to_fn_type(&item2));
        let key_expected = FnMetadata::to_hash_key(&expected);

        assert_eq!(key_expected, key1);
        assert_eq!(key_expected, key2);
    }
}
