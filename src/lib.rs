#![allow(unused)]

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::visit_mut::VisitMut;
use syn::*;

macro_rules! verbatim {
    (boxed, $($tokens:tt)*) => {
        Box::new(Expr::Verbatim(quote! { $($tokens)* }))
    };
    (option, $($tokens:tt)*) => {
        Some(Box::new(Expr::Verbatim(quote! { $($tokens)* })))
    };
    ($($tokens:tt)*) => {
        Expr::Verbatim(quote! { $($tokens)* })
    };
}

struct FnVisitor {
    fn_name: Ident,
}

impl VisitMut for FnVisitor {
    fn visit_expr_return_mut(&mut self, node: &mut ExprReturn) {
        node.expr = match node.expr.clone() {
            None => verbatim!(option, Action::Return(())),
            Some(some_expr) => Some(get_action_variant(*some_expr, self.fn_name.clone())),
        };
    }
}

fn get_action_variant(expr: Expr, fn_name: Ident) -> Box<Expr> {
    match expr {
        Expr::Call(expr_call) => {
            let func = expr_call.func.clone();
            let func_id: Ident = parse_quote!(#func);

            if func_id != fn_name {
                verbatim!(boxed, Action::Return(#expr_call))
            } else {
                let args = expr_call.args;
                verbatim!(boxed, Action::Continue((#args)))
            }
        }
        _ => verbatim!(boxed, Action::Return(#expr)),
    }
}

struct StmtVisitor {
    fn_name: Ident,
}

impl VisitMut for StmtVisitor {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        match node {
            Expr::Match(expr) => expr.arms.iter_mut().for_each(|arm| {
                arm.body = get_action_variant(*arm.body.clone(), self.fn_name.clone());
            }),
            _ => {
                *node = verbatim! { Action::Return(#node) };
            }
        }
    }
}

#[proc_macro_attribute]
pub fn recursive(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(item as ItemFn);
    let mut last_stmt = item.block.stmts.last().unwrap().clone();

    let mut fn_visitor = FnVisitor {
        fn_name: item.sig.ident.clone(),
    };
    let mut stmt_visitor = StmtVisitor {
        fn_name: item.sig.ident.clone(),
    };

    fn_visitor.visit_item_fn_mut(&mut item);
    stmt_visitor.visit_stmt_mut(&mut last_stmt);

    let fn_name = item.sig.ident.clone();
    let fn_name_inner = format_ident!("{}_inner", fn_name);
    let inputs = item.sig.inputs.clone();
    let input_pats = extract_fn_arg_pats(item.sig.inputs.clone());
    let input_types = extract_fn_arg_types(item.sig.inputs.clone());
    let return_type = extract_return_type(item.sig.output.clone());
    let mut fn_body = item.block.clone();

    let fn_body_last_stmt = fn_body.stmts.last_mut().unwrap();
    *fn_body_last_stmt = last_stmt;

    let iter_fn = quote! {
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
    };

    // println!("{}", iter_fn);

    iter_fn.into()
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
