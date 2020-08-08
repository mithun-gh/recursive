#![allow(unused)]

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::visit_mut::VisitMut;
use syn::*;

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

macro_rules! verbatim {
    (some, $($tokens:tt)*) => {
        Some(Box::new(Expr::Verbatim(quote! { $($tokens)* })))
    };
    (boxed, $($tokens:tt)*) => {
        Box::new(Expr::Verbatim(quote! { $($tokens)* }))
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
        transform_expr_return(node, &self.fn_name);
    }
}

struct StmtVisitor {
    fn_name: Ident,
}

impl VisitMut for StmtVisitor {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        transform_expr(node, &self.fn_name);
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

fn transform_expr_return(node: &mut ExprReturn, fn_name: &Ident) {
    node.expr = match node.expr.clone() {
        None => verbatim!(some, Action::Return(())),
        Some(some_expr) => Some(get_action_variant(*some_expr, fn_name.clone())),
    };
}

fn transform_expr(expr: &mut Expr, fn_name: &Ident) {
    match expr {
        Expr::Match(expr) => expr.arms.iter_mut().for_each(|arm| {
            arm.body = get_action_variant(*arm.body.clone(), fn_name.clone());
        }),
        Expr::If(expr) => {
            if let Some(last_stmt) = expr.then_branch.stmts.last_mut() {
                if let Stmt::Expr(expr) = last_stmt {
                    *expr = *get_action_variant(expr.clone(), fn_name.clone());
                }
            }
            if let Some((_, ref mut expr)) = &mut expr.else_branch {
                transform_expr(expr, &fn_name);
            }
        },
        Expr::Block(expr) => {
            if let Some(last_stmt) = expr.block.stmts.last_mut() {
                if let Stmt::Expr(expr) = last_stmt {
                    *expr = *get_action_variant(expr.clone(), fn_name.clone());
                }
            }
        },
        Expr::Return(expr) => {
            transform_expr_return(expr, &fn_name);
        },
        _ => {
            *expr = verbatim! { Action::Return(#expr) };
        }
    }
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
