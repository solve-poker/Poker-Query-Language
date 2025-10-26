use quote::quote;

use crate::{
    ArgMetadata, FnMetadata, RtnMetadata, id_to_pql_type,
    state::{is_already_implemented, mark_as_implemented},
};

type TokenStream2 = proc_macro2::TokenStream;

pub fn generate_pqlfn_impl(metadata: &FnMetadata) -> TokenStream2 {
    let ty = &metadata.fn_type;

    if is_already_implemented(ty) {
        return quote! {};
    }

    mark_as_implemented(ty);

    let arg_types_method = generate_arg_types_method(&metadata.args);
    let rtn_type_method = generate_rtn_type_method(&metadata.rtn);
    let execute_method = generate_execute_method(metadata);

    quote! {
        impl PQLFn for #ty {
            #arg_types_method
            #rtn_type_method
            #execute_method
        }
    }
}

fn generate_arg_types_method(args: &[ArgMetadata]) -> TokenStream2 {
    let mut arg_types = vec![];

    for arg in args {
        match arg {
            ArgMetadata::Context => (),
            ArgMetadata::StackValue(id) | ArgMetadata::HeapValue(id) => {
                arg_types.push(id_to_pql_type(id));
            }
        }
    }

    quote! {
        fn arg_types(&self) -> Vec<PQLType> {
            vec![ #( #arg_types ),* ]
        }
    }
}

fn generate_rtn_type_method(rtn: &RtnMetadata) -> TokenStream2 {
    let rtn_type = id_to_pql_type(&rtn.kind);

    quote! {
        fn rtn_type(&self) -> PQLType {
            #rtn_type
        }
    }
}

fn to_arg_def(i: usize, arg: &ArgMetadata) -> proc_macro2::TokenStream {
    let arg_name = quote::format_ident!("arg{i}");

    match arg {
        ArgMetadata::Context => {
            quote! { let #arg_name = &ctx.fn_ctx; }
        }
        ArgMetadata::StackValue(arg_type) => {
            quote! { let #arg_name = ctx.stack.downcast_pop::<#arg_type>(); }
        }
        ArgMetadata::HeapValue(arg_type) => {
            quote! {
               let i = ctx.stack.downcast_pop::<HeapIdx>();
               let #arg_name = ctx.heap.get_ref::<#arg_type>(i);
            }
        }
    }
}

fn generate_execute_method(meta: &FnMetadata) -> TokenStream2 {
    let arg_defs: Vec<_> = meta
        .args
        .iter()
        .enumerate()
        .rev()
        .map(|(i, arg)| to_arg_def(i, arg))
        .collect();

    let arg_names: Vec<_> = (0..meta.args.len())
        .map(|i| quote::format_ident!("arg{i}"))
        .collect();

    let fn_call = if meta.rtn.is_result {
        quote! { self(#(#arg_names),*)?.into() }
    } else {
        quote! { self(#(#arg_names),*).into() }
    };

    quote! {
        fn execute(&self, ctx: &mut VmExecContext) -> Result<VmStackValue, PQLErrorKind> {
            #(#arg_defs)*
            Ok(#fn_call)
        }
    }
}
