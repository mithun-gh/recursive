use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{fold::Fold, *};

mod extensions;

use crate::extensions::{ExprExtension, SignatureExtension};

#[proc_macro_attribute]
pub fn recursive(_attr: TokenStream, item_fn: TokenStream) -> TokenStream {
    let item_fn = parse_macro_input!(item_fn as ItemFn);
    let mut transformer = RecursionTransformer::new(item_fn);
    let item_fn = transformer.transform_item_fn();

    println!("{}", quote! { #item_fn });

    TokenStream::from(quote!(#item_fn))
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
    item_fn: ItemFn,
}

impl Fold for RecursionTransformer {
    fn fold_item_fn(&mut self, item_fn: ItemFn) -> ItemFn {
        let ItemFn { sig, block, .. } = item_fn;

        let fn_inner = format_ident!("{}_inner", sig.ident);
        let (input_pats, input_types) = sig.split_inputs();
        let return_type = sig.extract_return_type();

        let block = parse_quote! {{
            enum Action<C, R> {
                Continue(C),
                Return(R),
            }

            fn #fn_inner((#(#input_pats),*): (#(#input_types),*))
                -> Action<(#(#input_types),*), #return_type> #block

            let mut acc = (#(#input_pats),*);
            loop {
                match #fn_inner(acc) {
                    Action::Return(r) => return r,
                    Action::Continue(c) => acc = c,
                }
            }
        }};

        ItemFn {
            sig,
            block,
            ..item_fn
        }
    }
}

impl RecursionTransformer {
    fn new(item_fn: ItemFn) -> Self {
        RecursionTransformer { item_fn }
    }

    fn transform_item_fn(&mut self) -> ItemFn {
        let item_fn_new =
            ExprReturnTransformer::new(&self.item_fn.sig.ident).fold_item_fn(self.item_fn.clone());
        // let item_fn = &self.item_fn;

        // transform `return` expression
        // self.visit_item_fn_mut(&mut item_fn);

        // transform last expression
        // let mut last_stmt = item_fn.block.stmts.last().unwrap().clone();
        // self.visit_stmt_mut(&mut last_stmt);
        // let fn_body_last_stmt = item_fn.block.stmts.last_mut().unwrap();
        // *fn_body_last_stmt = last_stmt;

        self.fold_item_fn(item_fn_new)
    }
}

struct ExprReturnTransformer<'a> {
    fn_ident: &'a Ident,
}

impl<'a> ExprReturnTransformer<'a> {
    fn new(fn_ident: &'a Ident) -> Self {
        Self { fn_ident }
    }
}

impl<'a> Fold for ExprReturnTransformer<'a> {
    fn fold_expr_return(&mut self, expr_return: ExprReturn) -> ExprReturn {
        let ExprReturn { expr, .. } = expr_return;

        let expr = match expr {
            Some(expr) => expr.transform(self.fn_ident),
            None => verbatim! { Action::Return(()) },
        };

        ExprReturn {
            expr: Some(Box::new(expr)),
            ..expr_return
        }
    }
}
