use crate::imports::*;

pub fn basic_history_methods_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let ident = &ast.ident;
    let mut impl_block = TokenStream2::default();
    impl_block.extend::<TokenStream2>(quote! {
        impl #ident {
            /// Saves `self.state` to `self.history`
            pub fn save_state(&mut self) {
                self.history.push(self.state);
            }
        }
    });
    impl_block.into()
}

pub fn nested_history_methods_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let ident = &ast.ident;

    let fields: Vec<Field> = match ast.data {
        syn::Data::Struct(s) => s.fields.iter().map(|x| x.clone()).collect(),
        _ => panic!("only works on structs"),
    };

    // TODO: make this actually get used
    let _struct_has_state = fields
        .iter()
        .any(|x| x.ident.as_ref().unwrap().to_string() == "state");

    let has_state_vec: Vec<bool> = fields
        .iter()
        .map(|field| {
            field
                .attrs
                .iter()
                .any(|attr| attr.path.is_ident("has_state"))
        })
        .collect();

    let fields_with_state = fields
        .iter()
        .zip(has_state_vec)
        .filter(|(_f, hsv)| *hsv)
        .map(|(f, _hsv)| f.ident.as_ref().unwrap())
        .collect::<Vec<_>>();

    let mut impl_block = TokenStream2::default();
    impl_block.extend::<TokenStream2>(quote! {
        impl #ident {
            /// Saves `self.state` to `self.history`
            pub fn save_state(&mut self) {
                self.history.push(self.state);
                #(self.#fields_with_state.save_state();)*
            }
        }
    });
    impl_block.into()
}
