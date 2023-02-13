use crate::imports::*;

pub(crate) fn bare_clone_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let ident = &ast.ident;

    let fields: Vec<Field> = match ast.data {
        syn::Data::Struct(s) => s.fields.iter().map(|x| x.clone()).collect(),
        _ => panic!("only works on structs"),
    };

    // all fields with `has_state` or `history` attribute
    let has_bare_clone: Vec<bool> = fields
        .iter()
        .map(|field| {
            field
                .attrs
                .iter()
                .any(|attr| attr.path.is_ident("has_state") || attr.path.is_ident("history"))
        })
        .collect();

    // vec of fields that should be cloned with `bare_clone()`
    let fields_to_bare_clone = fields
        .iter()
        .zip(has_bare_clone.clone())
        .filter(|(_f, hbc)| *hbc)
        .map(|(f, _hsv)| f.ident.as_ref().unwrap())
        .collect::<Vec<_>>();

    let is_not_history: Vec<bool> = fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap().to_string() != "history")
        .collect();

    // vec of fields to be cloned with `clone()`
    let mut fields_to_clone = fields
        .iter()
        .zip(is_not_history.clone())
        .filter(|(_f, ih)| *ih)
        .map(|(f, _ih)| f.ident.as_ref().unwrap())
        .collect::<Vec<_>>();
    // purge out all the elements in `fields_to_bare_clone`
    fields_to_clone.retain(|&x| !fields_to_bare_clone.contains(&x));

    let mut impl_block = TokenStream2::default();

    impl_block.extend::<TokenStream2>(quote! {
        impl #ident {
            /// Returns a copy
            pub fn bare_clone(&self) -> Self {
                let mut new = Self::default();
                #(new.#fields_to_clone = self.#fields_to_clone.clone();)*
                new
            }
        }
    });

    impl_block.into()
}
