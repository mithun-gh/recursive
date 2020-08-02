use proc_macro::TokenStream;
use quote::quote;
use syn::visit_mut::VisitMut;
use syn::*;

mod utils;

#[allow(unused)]
#[derive(Debug)]
enum Action<C, R> {
    Continue(C),
    Return(R),
}

#[allow(unused)]
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
    let item: ItemFn = syn::parse(item).unwrap();

    let input_pats = utils::extract_fn_arg_pats(item.sig.inputs.clone());
    let input_types = utils::extract_fn_arg_types(item.sig.inputs.clone());

    input_pats.iter().for_each(|i| println!("? {:?}", i));
    input_types.iter().for_each(|i| println!("! {:?}", i));

    // FnVisitor.visit_item_fn_mut(&mut item);

    (quote! { #item }).into()
}
