#![allow(clippy::wildcard_imports)]

use super::*;

pub struct RtnMetadata {
    pub kind: Ident,
    pub is_result: bool,
}

impl RtnMetadata {
    pub fn new(rtn: &ReturnType) -> Self {
        match rtn {
            ReturnType::Default => panic!("() not supported"),
            ReturnType::Type(_, ty) => Self::from_type(ty),
        }
    }

    fn from_type(ty: &Type) -> Self {
        match ty {
            Type::Path(ty_path) => {
                let kind = typepath_to_id(ty_path);

                if kind == "Result" {
                    Self {
                        kind: Self::get_result_ok_ident(ty_path),
                        is_result: true,
                    }
                } else {
                    Self {
                        kind,
                        is_result: false,
                    }
                }
            }
            _ => panic!("{} not supported", ty.to_token_stream()),
        }
    }

    fn get_result_ok_ident(ty_path: &TypePath) -> Ident {
        fn inner(ty_path: &TypePath) -> Option<Ident> {
            ty_path
                .path
                .segments
                .last()
                .and_then(|segment| match &segment.arguments {
                    PathArguments::AngleBracketed(angle_args) => {
                        angle_args.args.first()
                    }
                    _ => None,
                })
                .and_then(|arg| match arg {
                    GenericArgument::Type(Type::Path(inner)) => {
                        Some(typepath_to_id(inner))
                    }
                    _ => None,
                })
        }

        inner(ty_path).unwrap_or_else(|| {
            panic!("{} not supported", ty_path.to_token_stream())
        })
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "() not supported")]
    fn test_unit_return_type_panics() {
        let rtn = ReturnType::Default;
        RtnMetadata::new(&rtn);
    }

    #[test]
    #[should_panic(expected = "not supported")]
    fn test_reference_return_type_panics() {
        let rtn: ReturnType = parse_quote! { -> &str };
        RtnMetadata::new(&rtn);
    }

    #[test]
    #[should_panic(expected = "not supported")]
    fn test_tuple_return_type_panics() {
        let rtn: ReturnType = parse_quote! { -> (String, i32) };
        RtnMetadata::new(&rtn);
    }

    #[test]
    #[should_panic(expected = "not supported")]
    fn test_result_without_generics_panics() {
        let rtn: ReturnType = parse_quote! { -> Result };
        RtnMetadata::new(&rtn);
    }
}
