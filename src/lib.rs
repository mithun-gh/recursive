#![allow(unused)]

use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::visit_mut::VisitMut;
use syn::*;

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

    let fn_name =  item.sig.ident.clone();
    let fn_name_inner = format_ident!("{}_inner", fn_name);
    let inputs = item.sig.inputs.clone();
    let input_pats = extract_fn_arg_pats(item.sig.inputs.clone());
    let input_types = extract_fn_arg_types(item.sig.inputs.clone());
    let return_type = extract_return_type(item.sig.output.clone());
    let fn_body = item.block.clone();

    if return_type.is_some() {
        println!("{}", quote! {
            fn #fn_name(#inputs) -> #return_type {
                enum Action<C, R> {
                    Continue(C),
                    Return(R),
                }

                fn #fn_name_inner((#(#input_pats),*): (#(#input_types),*))
                    -> Action<(#(#input_types),*), #return_type>
                    #fn_body

                let mut acc = (#(#input_pats),*);
                loop {
                    match #fn_name_inner(acc) {
                        Action::Return(r) => return r,
                        Action::Continue(c) => acc = c,
                    }
                }
            }
        });
    }

    // FnVisitor.visit_item_fn_mut(&mut item);

    (quote! { #item }).into()
}

fn extract_fn_arg_pat(arg: FnArg) -> Pat {
    match arg {
        FnArg::Typed(pt) => *pt.pat,
        _ => panic!("not supported on the receiver type `self`"),
    }
}

fn extract_fn_arg_pats(args: Punctuated<FnArg, Comma>) -> Vec<Pat> {
    args.into_iter().map(extract_fn_arg_pat).collect::<Vec<_>>()
}

fn extract_fn_arg_type(arg: FnArg) -> Type {
    match arg {
        FnArg::Typed(pt) => *pt.ty,
        _ => panic!("not supported on the receiver type `self`"),
    }
}

fn extract_fn_arg_types(args: Punctuated<FnArg, Comma>) -> Vec<Type> {
    args.into_iter()
        .map(extract_fn_arg_type)
        .collect::<Vec<_>>()
}

fn extract_return_type(output: ReturnType) -> Option<Type> {
    match output {
        ReturnType::Type(_, return_type) => Some(*return_type),
        ReturnType::Default => None
    }
}
