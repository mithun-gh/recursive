use proc_macro::{TokenStream, Span};
use quote::quote;
use syn::*;
use syn::visit_mut::VisitMut;

#[allow(unused)]
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
        let t = make!(ReturnType, -> String);
        println!(">> {:?}", t);
        *return_ty = t;
    }
}

#[proc_macro_attribute]
pub fn recursive(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item: ItemFn = syn::parse(item).unwrap();

    FnVisitor.visit_item_fn_mut(&mut item);

    (quote!{ #item }).into()
}

fn _make_ident(name: &str) -> Ident {
    Ident::new(name, Span::call_site().into())
}
