#![allow(clippy::wildcard_imports)]

use super::*;

pub struct AttrItem {
    ident: Ident,
    value: Option<LitStr>,
}

impl Parse for AttrItem {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;
        let value = if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            Some(input.parse::<LitStr>()?)
        } else {
            None
        };

        Ok(Self { ident, value })
    }
}

pub struct AttrItems {
    items: Punctuated<AttrItem, Token![,]>,
}

impl AttrItems {
    pub fn into_meta(self) -> (bool, Option<String>) {
        let mut no_parse = false;
        let mut alias = None;

        for attr_item in self.items {
            let id = attr_item.ident;
            if id == "no_parse" {
                no_parse = true;
            } else if id == "alias" {
                alias = attr_item
                    .value
                    .expect("expect alias string literal")
                    .value()
                    .into();
            } else {
                panic!("attribute {id} not supported");
            }
        }

        (no_parse, alias)
    }
}

impl Parse for AttrItems {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            items: input.parse_terminated(AttrItem::parse, Token![,])?,
        })
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    #[should_panic(expected = "expect alias string literal")]
    fn test_alias_without_value_panics() {
        let items: AttrItems = parse_quote! { alias };
        items.into_meta();
    }
    #[test]
    #[should_panic(expected = "not supported")]
    fn test_unknown_attr() {
        let items: AttrItems = parse_quote! { invalid };
        items.into_meta();
    }

    #[test]
    fn test_parse_invalid_syntax() {
        let result = syn::parse2::<AttrItems>(quote::quote! { = });
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_incomplete_assignment() {
        let result = syn::parse2::<AttrItems>(quote::quote! { alias = });
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_alias() {
        let result = syn::parse2::<AttrItems>(quote::quote! { alias? });
        assert!(result.is_err());
    }

    #[test]
    fn test_no_parse_returns_none_alias() {
        let items: AttrItems = parse_quote! { no_parse };
        let (no_parse, alias) = items.into_meta();
        assert!(no_parse);
        assert_eq!(alias, None);
    }

    #[test]
    fn test_empty_items_returns_defaults() {
        let items: AttrItems = parse_quote! {};
        let (no_parse, alias) = items.into_meta();
        assert!(!no_parse);
        assert_eq!(alias, None);
    }

    #[test]
    fn test_alias_with_value_returns_some() {
        let items: AttrItems = parse_quote! { alias = "test_alias" };
        let (no_parse, alias) = items.into_meta();
        assert!(!no_parse);
        assert_eq!(alias, Some("test_alias".to_string()));
    }
}
