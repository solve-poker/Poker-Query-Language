#![allow(clippy::wildcard_imports)]

use super::*;

pub enum ArgMetadata {
    Context,
    StackValue(Ident),
    HeapValue(Ident),
}

impl ArgMetadata {
    pub fn new(arg: &FnArg) -> Self {
        let arg_type = Self::get_type(arg);

        match arg_type {
            Type::Path(ty_path) => Self::StackValue(typepath_to_id(ty_path)),

            Type::Reference(ty_ref) => {
                let ident = type_to_id(&ty_ref.elem);

                if ident == "PQLFnContext" {
                    Self::Context
                } else {
                    Self::HeapValue(ident)
                }
            }

            _ => panic!("{} not supported", arg_type.to_token_stream()),
        }
    }

    fn get_type(arg: &FnArg) -> &Type {
        match arg {
            FnArg::Receiver(_) => panic!("self not supported"),
            FnArg::Typed(arg) => &arg.ty,
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "impl Iterator not supported")]
    fn test_impl_arg() {
        let arg: FnArg = parse_quote! { _: impl Iterator };
        ArgMetadata::new(&arg);
    }

    #[test]
    #[should_panic(expected = "self not supported")]
    fn test_self_arg() {
        let arg: FnArg = parse_quote! { self };
        ArgMetadata::new(&arg);
    }
}
