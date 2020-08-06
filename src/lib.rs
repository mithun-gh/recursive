#![allow(unused)]

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::visit_mut::VisitMut;
use syn::*;

macro_rules! verbatim {
    ($($tokens:tt)*) => {
        Some(Box::new(Expr::Verbatim(quote! { $($tokens)* })))
    };
}

struct FnVisitor;

impl VisitMut for FnVisitor {
    fn visit_expr_return_mut(&mut self, node: &mut ExprReturn) {
        node.expr = match node.expr.clone() {
            None => verbatim! { Action::Return(()) },
            Some(some_expr) => match *some_expr {
                Expr::Call(expr_call) => {
                    let args = expr_call.args;
                    verbatim! { Action::Continue((#args)) }
                }
                _ => {
                    verbatim! { Action::Return(#some_expr) }
                }
            },
        };
    }
}

#[proc_macro_attribute]
pub fn recursive(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(item as ItemFn);

    FnVisitor.visit_item_fn_mut(&mut item);

    let fn_name = item.sig.ident.clone();
    let fn_name_inner = format_ident!("{}_inner", fn_name);
    let inputs = item.sig.inputs.clone();
    let input_pats = extract_fn_arg_pats(item.sig.inputs.clone());
    let input_types = extract_fn_arg_types(item.sig.inputs.clone());
    let return_type = extract_return_type(item.sig.output.clone());
    let fn_body = item.block.clone();

    // let mut last_stmt = item.block.stmts.last().unwrap().clone();

    (quote! {
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
    })
    .into()
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

fn extract_return_type(output: ReturnType) -> Type {
    match output {
        ReturnType::Default => Type::Verbatim(quote! { () }),
        ReturnType::Type(_, return_type) => *return_type,
    }
}
