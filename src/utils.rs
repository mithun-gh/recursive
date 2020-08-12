use syn::*;

pub trait SignatureExtensions {
    fn split_inputs(&self) -> (Vec<Pat>, Vec<Type>);
    fn extract_return_type(&self) -> Type;
}

impl SignatureExtensions for Signature {
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
                _ => unimplemented!("receiver type `self` is not supported yet."),
            })
    }

    fn extract_return_type(&self) -> Type {
        match self.output.clone() {
            ReturnType::Default => parse_quote!(()),
            ReturnType::Type(_, return_type) => *return_type,
        }
    }
}
