use proc_macro::TokenStream;
use quote::quote;
use syn::{self, ItemFn, ExprReturn, visit_mut::VisitMut};

struct FnVisitor;

impl VisitMut for FnVisitor {
    fn visit_expr_return_mut(&mut self, expr_return: &mut ExprReturn) {
        println!("{:?}\n", expr_return.expr)
    }
}

#[proc_macro_attribute]
pub fn tail_recursive(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item: ItemFn = syn::parse(item).unwrap();

    FnVisitor.visit_item_fn_mut(&mut item);

    (quote! { #item }).into()
}
