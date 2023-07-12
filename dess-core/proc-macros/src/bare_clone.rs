use crate::imports::*;

///
pub(crate) fn bare_clone_derive(input: TokenStream) -> TokenStream {
    let item_struct = syn::parse_macro_input!(input as syn::ItemStruct);
    let ident = &item_struct.ident;
    let fields = item_struct.fields;

    // all fields with `use_state` or `save_state` attribute
    let has_bare_clone: Vec<bool> = fields
        .iter()
        .map(|field| {
            field
                .attrs
                .iter()
                .any(|attr| attr.path.is_ident("use_state") || attr.path.is_ident("save_state"))
        })
        .collect();

    // vec of fields that should be cloned with `bare_clone()`
    let fields_to_bare_clone = fields
        .iter()
        .zip(has_bare_clone)
        .filter(|(_f, hbc)| *hbc)
        .map(|(f, _hsv)| f.ident.as_ref().unwrap())
        .collect::<Vec<_>>();

    let is_not_history: Vec<bool> = fields
        .iter()
        .map(|field| *field.ident.as_ref().unwrap() != "history")
        .collect();

    // vec of fields to be cloned with `clone()`
    let mut fields_to_clone = fields
        .iter()
        .zip(is_not_history)
        .filter(|(_f, ih)| *ih)
        .map(|(f, _ih)| f.ident.as_ref().unwrap())
        .collect::<Vec<_>>();
    // purge out all the elements in `fields_to_bare_clone`
    fields_to_clone.retain(|&x| !fields_to_bare_clone.contains(&x));

    let mut impl_block = TokenStream2::default();

    impl_block.extend::<TokenStream2>(quote! {
        impl dess_core::traits_and_macros::BareClone for #ident {
            /// Returns a copy
            fn bare_clone(&self) -> Self {
                let mut new = Self::default();
                #(new.#fields_to_bare_clone = self.#fields_to_bare_clone.bare_clone();)*
                #(new.#fields_to_clone = self.#fields_to_clone.clone();)*
                new
            }
        }
    });

    impl_block.into()
}
