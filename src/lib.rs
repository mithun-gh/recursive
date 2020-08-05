#![allow(unused)]

use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::visit_mut::VisitMut;
use syn::*;

#[derive(Debug)]
enum Action<C, R> {
    Continue(C),
    Return(R),
}

macro_rules! make {
    ($make_type:ty, $($tokens:tt)+) => {
        parse::<$make_type>(TokenStream::from(quote! { $($tokens)+ })).unwrap()
    };
}

struct FnVisitor;

impl VisitMut for FnVisitor {
    fn visit_return_type_mut(&mut self, return_ty: &mut ReturnType) {
        // let t = make!(ReturnType, -> String);

        println!("!!! {:?}", return_ty);

        if let ReturnType::Type(_, type_details) = return_ty {
            println!("!!! {}", quote!(-> Action<#type_details>));
        }

        // *return_ty = t;
    }
}

#[proc_macro_attribute]
pub fn recursive(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);

    // let input_pats = utils::extract_fn_arg_pats(item.sig.inputs.clone());
    // let input_types = utils::extract_fn_arg_types(item.sig.inputs.clone());

    // let tuple: TypeTuple = parse_quote! {
    //     (#(input_types),+)
    // };

    // input_pats.iter().for_each(|i| println!("? {:?}", i));
    // input_types.iter().for_each(|i| println!("! {:?}", i));

    // FnVisitor.visit_item_fn_mut(&mut item);

    (quote! { #item }).into()
}

fn extract_fn_arg_pat(arg: FnArg) -> Box<Pat> {
    match arg {
        FnArg::Typed(pt) => pt.pat,
        _ => panic!("not supported on the receiver type `self`"),
    }
}

fn extract_fn_arg_pats(args: Punctuated<FnArg, Comma>) -> Vec<Box<Pat>> {
    args.into_iter().map(extract_fn_arg_pat).collect::<Vec<_>>()
}

fn extract_fn_arg_type(arg: FnArg) -> Box<Type> {
    match arg {
        FnArg::Typed(pt) => pt.ty,
        _ => panic!("not supported on the receiver type `self`"),
    }
}

fn extract_fn_arg_types(args: Punctuated<FnArg, Comma>) -> Vec<Box<Type>> {
    args.into_iter()
        .map(extract_fn_arg_type)
        .collect::<Vec<_>>()
}
