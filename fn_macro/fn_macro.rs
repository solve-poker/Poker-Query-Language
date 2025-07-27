use std::sync::Mutex;

use lazy_static::lazy_static;
use proc_macro::TokenStream;
use quote::*;
use syn::*;

type TokenStream2 = proc_macro2::TokenStream;

lazy_static! {
    static ref ARMS: Mutex<Vec<String>> = Mutex::new(vec!());
}

#[proc_macro_attribute]
pub fn pqlfn(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = attr.to_string();

    let (args, rtn, eval_args, is_rtn_result, ty) = transform_fn(item.clone());

    let impl_arg = if attr.contains("arg") {
        gen_impl_arg(&ty, &args)
    } else {
        TokenStream2::new()
    };

    let impl_rtn = if attr.contains("rtn") {
        gen_impl_rtn(&ty, &rtn)
    } else {
        TokenStream2::new()
    };

    let impl_eval = if attr.contains("eval") {
        gen_impl_eval(&ty, &eval_args, is_rtn_result)
    } else {
        TokenStream2::new()
    };

    let fndef = TokenStream2::from(item);

    quote! {
        #fndef

        #impl_eval
        #impl_arg
        #impl_rtn
    }
    .into()
}

#[proc_macro]
pub fn pqlfn_fromstr(_item: TokenStream) -> TokenStream {
    let arms: Vec<TokenStream2> = ARMS
        .lock()
        .unwrap()
        .iter()
        .map(|ln| ln.parse().unwrap())
        .collect();

    quote! {
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_ascii_lowercase().as_str() {
                #(#arms,)*

                _ => Err(ParseError::InvalidFunctionName(s.into())),
            }
        }
    }
    .into()
}

fn gen_impl_arg(ty: &TokenStream2, args: &[TokenStream2]) -> TokenStream2 {
    quote! {
        impl PQLFnArgs for #ty {
            fn arg_types(&self) -> &[PQLType] {
                &[#(#args),*]
            }
        }
    }
}

fn gen_impl_rtn(ty: &TokenStream2, rtn: &TokenStream2) -> TokenStream2 {
    quote! {
        impl PQLFnRtn for #ty {
            fn rtn_type(&self) -> PQLType {
                #rtn
            }
        }
    }
}

fn gen_impl_eval(
    ty: &TokenStream2,
    eval_args: &[TokenStream2],
    is_rtn_result: bool,
) -> TokenStream2 {
    let question_mark = if is_rtn_result {
        quote! {?}
    } else {
        quote! {}
    };

    quote! {
        impl PQLFnEval for #ty {
            fn evaluate(
                &self,
                buffer: &mut VmBuffer,
                store: &mut VmStore,
                stack: &mut VmStack,
            ) -> Result<VmStackValue, PQLError> {
                Ok(self(#(#eval_args),*)#question_mark.into())
            }
        }
    }
}

fn transform_fn(
    input: TokenStream,
) -> (
    Vec<TokenStream2>,
    TokenStream2,
    Vec<TokenStream2>,
    bool,
    TokenStream2,
) {
    let input_fn: ItemFn =
        syn::parse2(input.into()).expect("Failed to parse token stream");

    let inputs = &input_fn.sig.inputs;
    let output = &input_fn.sig.output;

    let input_types: Vec<Type> = inputs
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                Some((*pat_type.ty).clone())
            } else {
                None
            }
        })
        .collect();

    let kind_inputs = input_types.iter().map(to_pql_type).collect();
    let eval_args = input_types.iter().map(to_eval_arg).collect();
    let kind_output = rtn_to_type(output);
    let is_result = output.to_token_stream().to_string().contains("Result");

    let ty = quote! { fn(#(#input_types),*) #output };

    let fnname = input_fn.sig.ident.clone();
    let pqlfnname = to_pql_fn_name(&fnname.to_string());
    let match_arm = quote! {
        #pqlfnname => Ok(&(#fnname as #ty))
    };

    ARMS.lock().unwrap().push(match_arm.to_string());

    (kind_inputs, kind_output, eval_args, is_result, ty)
}

fn to_pql_fn_name(s: &str) -> String {
    s.chars().filter(|c| c.is_alphanumeric()).collect()
}

fn rtn_to_type(ty: &ReturnType) -> TokenStream2 {
    if let ReturnType::Type(_, ty) = ty {
        to_pql_type(ty)
    } else {
        panic!("-> () not implemented")
    }
}

fn to_eval_arg(t: &Type) -> TokenStream2 {
    use quote as q;
    let s = type_to_string(t);

    match s.as_str() {
        "Hand" => q! { arg_player(buffer, store, stack) },
        "str" => q! { arg_str(buffer, store, stack) },

        "PQLRange" | "PQLBoardRange" => q! { arg_range(buffer, store, stack) },

        "PQLCardCount"
        | "PQLFlopHandCategory"
        | "PQLHandType"
        | "PQLHiRating"
        | "PQLInteger"
        | "PQLPlayer"
        | "PQLRankSet"
        | "PQLStreet" => q! { stack.downcast_pop().unwrap() },

        "Board"
        | "PQLGame"
        | "(PQLGame, Flop)"
        | "(PQLGame, Board)"
        | "(PQLGame, Board, DeadCards)" => q! { (&*buffer).into() },

        "(PQLGame, Board, &PlayerHands, &mut BufferRatings)" => {
            q! { buffer.into() }
        }

        _ => unimplemented!("{}", s),
    }
}

fn to_pql_type(t: &Type) -> TokenStream2 {
    use quote as q;
    let s = type_to_string(t);

    match s.as_str() {
        "Hand" => q! { PQLType::Player },
        "str" => q! { PQLType::String },

        "PQLBoardRange" => q! { PQLType::BoardRange },
        "PQLBoolean" => q! { PQLType::Boolean },
        "PQLCard" => q! { PQLType::Card },
        "PQLCardCount" => q! { PQLType::CardCount },
        "PQLDouble" => q! { PQLType::Double },
        "PQLFlopHandCategory" => q! { PQLType::FlopHandCategory },
        "PQLHandType" => q! { PQLType::HandType },
        "PQLHiRating" => q! { PQLType::HiRating },
        "PQLInteger" => q! { PQLType::Integer },
        "PQLPlayer" => q! { PQLType::Player },
        "PQLRange" => q! { PQLType::Range },
        "PQLRank" => q! { PQLType::Rank },
        "PQLRankSet" => q! { PQLType::RankSet },
        "PQLStreet" => q! { PQLType::Street },

        "Board"
        | "PQLGame"
        | "(PQLGame, Board)"
        | "(PQLGame, Flop)"
        | "(PQLGame, Board, &PlayerHands, &mut BufferRatings)"
        | "(PQLGame, Board, DeadCards)" => q! {},

        _ => unimplemented!("{}", s),
    }
}

fn type_to_string(t: &Type) -> String {
    if let Type::Path(ty) = t {
        strip_path(ty)
    } else if let Type::Reference(ty) = t {
        type_to_string(&ty.elem)
    } else if let Type::Tuple(ty) = t {
        ty.to_token_stream()
            .to_string()
            .replace(" ,", ",")
            .replace("& ", "&")
            .replace(",)", ")")
    } else {
        unimplemented!("{:?}", t.to_token_stream().to_string());
    }
}

fn strip_path(t: &TypePath) -> String {
    if &t.path.segments[0].ident == "Result" {
        return strip_opt_result(&t.path.segments[0].arguments);
    }

    if &t.path.segments[0].ident == "Option" {
        return strip_opt_result(&t.path.segments[0].arguments);
    }

    match t.path.get_ident() {
        Some(id) => id.to_string(),
        None => unimplemented!("{:?}", t.to_token_stream().to_string()),
    }
}

fn strip_opt_result(t: &PathArguments) -> String {
    if let PathArguments::AngleBracketed(a) = t {
        if let GenericArgument::Type(t) = &a.args[0] {
            return type_to_string(t);
        }
    }

    unimplemented!("{:?}", t.to_token_stream().to_string());
}
