use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{fold::Fold, visit_mut::VisitMut, *};

mod utils;

use crate::utils::SignatureExtensions;

#[proc_macro_attribute]
pub fn recursive(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(item as ItemFn);

    let mut transformer = RecursionTransformer {
        fn_name: item.sig.ident.clone(),
    };

    transformer.visit_item_fn_mut(&mut item);

    let mut last_stmt = item.block.stmts.last().unwrap().clone();

    transformer.visit_stmt_mut(&mut last_stmt);

    let fn_name = item.sig.ident.clone();
    let fn_name_inner = format_ident!("{}_inner", fn_name);
    let inputs = item.sig.inputs.clone();
    let (input_pats, input_types) = item.sig.split_inputs();
    let return_type = item.sig.extract_return_type();
    let mut fn_body = item.block;

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

struct RecursionTransformer {
    fn_name: Ident,
}

impl Fold for RecursionTransformer {
    fn fold_item_fn(&mut self, item_fn: ItemFn) -> ItemFn {
        let ItemFn {
            attrs,
            vis,
            sig,
            block,
        } = item_fn;

        ItemFn {
            attrs,
            vis,
            sig,
            block,
        }
    }
}

impl RecursionTransformer {
    fn transform_expr_return(&self, node: &mut ExprReturn) {
        if let Some(ref mut some_expr) = node.expr {
            self.transform_expr(some_expr);
        } else {
            node.expr = verbatim!(some, Action::Return(()));
        }
    }

    fn transform_expr(&self, expr: &mut Expr) {
        match expr {
            Expr::Call(expr_call) => {
                let func = expr_call.func.clone();
                let func_id: Ident = parse_quote!(#func);

                if func_id != self.fn_name {
                    *expr = verbatim! { Action::Return(#expr_call) };
                } else {
                    let args = expr_call.args.clone();
                    *expr = verbatim! { Action::Continue((#args)) };
                }
            }
            Expr::Match(expr) => expr.arms.iter_mut().for_each(|arm| {
                self.transform_expr(&mut arm.body);
            }),
            Expr::If(expr) => {
                if let Some(last_stmt) = expr.then_branch.stmts.last_mut() {
                    if let Stmt::Expr(expr) = last_stmt {
                        self.transform_expr(expr);
                    }
                }
                if let Some((_, ref mut expr)) = &mut expr.else_branch {
                    self.transform_expr(expr);
                }
            }
            Expr::Block(expr) => {
                if let Some(last_stmt) = expr.block.stmts.last_mut() {
                    if let Stmt::Expr(expr) = last_stmt {
                        self.transform_expr(expr);
                    }
                }
            }
            Expr::Return(_) | Expr::Verbatim(_) => {
                // Ignore return expressions as they are handled separately.
                // And, ignore verbatim!{} expressions generated by this macro.
            }
            _ => {
                *expr = verbatim! { Action::Return(#expr) };
            }
        }
    }
}

impl VisitMut for RecursionTransformer {
    fn visit_expr_return_mut(&mut self, node: &mut ExprReturn) {
        self.transform_expr_return(node);
    }

    fn visit_expr_mut(&mut self, node: &mut Expr) {
        self.transform_expr(node);
    }
}
