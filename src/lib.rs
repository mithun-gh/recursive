use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Ident, ItemFn, visit_mut::VisitMut};

struct FnRenamer(String);

impl VisitMut for FnRenamer {
    fn visit_ident_mut(&mut self, ident: &mut Ident) {
        if ident == self.0.as_str() {
            *ident = Ident::new(format!("__internal_{}_rec", self.0).as_str(), ident.span());
        }
    }
}

#[proc_macro_attribute]
pub fn recursive(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item: ItemFn = syn::parse(item).expect("parser error");

    FnRenamer(item.sig.ident.to_string()).visit_item_fn_mut(&mut item);

    (quote! { #item }).into()
}
