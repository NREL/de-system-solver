use crate::imports::*;

// taken from https://github.com/lumol-org/soa-derive/blob/master/soa-derive-internal/src/input.rs
pub(crate) trait TokenStreamIterator {
    fn concat_by(
        self,
        f: impl Fn(proc_macro2::TokenStream, proc_macro2::TokenStream) -> proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream;
    fn concat(self) -> proc_macro2::TokenStream;
}

impl<T: Iterator<Item = proc_macro2::TokenStream>> TokenStreamIterator for T {
    fn concat_by(
        mut self,
        f: impl Fn(proc_macro2::TokenStream, proc_macro2::TokenStream) -> proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        match self.next() {
            Some(first) => self.fold(first, f),
            None => quote! {},
        }
    }

    fn concat(self) -> proc_macro2::TokenStream {
        self.concat_by(|a, b| quote! { #a #b })
    }
}

#[allow(unused)]
/// Checks if a field is a vec
pub fn is_vec(field: &Field) -> bool {
    if let Type::Path(type_path) = &field.ty {
        if let Some(segment) = type_path.path.segments.first() {
            if segment.ident == "Vec" {
                return true;
            }
        }
    }
    false
}

/// accepts `attr` TokenStream from attribute-like proc macro and returns
/// TokenStream2 of fn defs that are in `expected_fn_names` and/or not in `forbidden_fn_names`.  
/// If `expected_exlusive` is true, only values in `expected_fn_names` are allowed.  
/// Raises locationally useful errors if mistakes are made in formatting or whatnot.  
pub fn parse_ts_as_fn_defs(
    attr: TokenStream,
    mut expected_fn_names: Vec<String>,
    expected_exclusive: bool,
    forbidden_fn_names: Vec<String>,
) -> TokenStream2 {
    let attr = TokenStream2::from(attr);
    let impl_block = quote! {
        impl Dummy { // this name doesn't really matter as it won't get used
            #attr
        }
    }
    .into();
    // let item_impl = syn::parse_macro_input!(impl_block as syn::ItemImpl);
    let item_impl = syn::parse::<syn::ItemImpl>(impl_block)
        .map_err(|_| abort_call_site!("Only function definitions allowed here."))
        .unwrap();

    let mut fn_from_attr = TokenStream2::new();

    for impl_item in item_impl.items {
        match &impl_item {
            syn::ImplItem::Method(item_meth) => {
                let sig_str = &item_meth.sig.ident.to_token_stream().to_string();
                fn_from_attr.extend(item_meth.clone().to_token_stream());
                // check signature
                if expected_exclusive {
                    if forbidden_fn_names.contains(sig_str) || !expected_fn_names.contains(sig_str)
                    {
                        abort!(
                            &item_meth.sig.ident.span(),
                            format!("{} is forbidden", sig_str)
                        )
                    }
                }

                let index = expected_fn_names.iter().position(|x| x == sig_str);

                match index {
                    Some(i) => {
                        expected_fn_names.remove(i);
                    }
                    _ => {}
                }
                // remove the matching name from the vec to avoid checking again
                // at the end of iteration, this vec should be empty
            }
            _ => abort_call_site!("Expected only method definitions in `solver` argument"),
        }
    }

    if !expected_fn_names.is_empty() {
        abort_call_site!(format!("Expected fn def for {:?}", expected_fn_names));
    }
    fn_from_attr
}
