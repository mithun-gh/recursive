use quote::quote;
use syn::*;

pub trait SignatureExtension {
    fn split_inputs(&self) -> (Vec<Pat>, Vec<Type>);
    fn extract_return_type(&self) -> Type;
}

impl SignatureExtension for Signature {
    fn split_inputs(&self) -> (Vec<Pat>, Vec<Type>) {
        self.inputs
            .clone()
            .into_iter()
            .fold((vec![], vec![]), |mut acc, arg| match arg {
                FnArg::Typed(pt) => {
                    acc.0.push(*pt.pat);
                    acc.1.push(*pt.ty);
                    (acc.0, acc.1)
                }
                FnArg::Receiver(_) => acc,
            })
    }

    fn extract_return_type(&self) -> Type {
        match self.output.clone() {
            ReturnType::Default => parse_quote!(()),
            ReturnType::Type(_, return_type) => *return_type,
        }
    }
}

macro_rules! transform_call_expr {
    ($left:expr, $right:expr, $call_expr:ident) => {
        if $left != $right {
            Expr::Verbatim(quote! { Action::Return(#$call_expr) })
        } else {
            let args = &$call_expr.args;
            Expr::Verbatim(quote! { Action::Continue((#args)) })
        }
    };
}

pub trait ExprExtension {
    fn transform(&self, source_fn_ident: &Ident) -> Expr;
}

impl ExprExtension for Expr {
    fn transform(&self, source_fn_ident: &Ident) -> Expr {
        match self {
            Expr::Call(expr_call) => {
                let fn_ident = &*expr_call.func;
                let fn_ident: Ident = parse_quote!(#fn_ident);
                transform_call_expr!(fn_ident, *source_fn_ident, expr_call)
            }
            Expr::MethodCall(expr_method_call) => {
                let fn_ident = &expr_method_call.method;
                transform_call_expr!(*fn_ident, *source_fn_ident, expr_method_call)
            }
            _ => Expr::Verbatim(quote! { Action::Return(#self) }),
        }
    }
}
