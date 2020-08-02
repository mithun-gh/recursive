use syn::*;
use syn::punctuated::Punctuated;
use syn::token::Comma;

pub fn extract_fn_arg_pat(arg: FnArg) -> Box<Pat> {
    match arg {
        FnArg::Typed(pt) => pt.pat,
        _ => panic!("not supported on the receiver type `self`"),
    }
}

pub fn extract_fn_arg_pats(args: Punctuated<FnArg, Comma>) -> Vec<Box<Pat>> {
    args.into_iter().map(extract_fn_arg_pat).collect::<Vec<_>>()
}

pub fn extract_fn_arg_type(arg: FnArg) -> Box<Type> {
    match arg {
        FnArg::Typed(pt) => pt.ty,
        _ => panic!("not supported on the receiver type `self`"),
    }
}

pub fn extract_fn_arg_types(args: Punctuated<FnArg, Comma>) -> Vec<Box<Type>> {
    args.into_iter().map(extract_fn_arg_type).collect::<Vec<_>>()
}
