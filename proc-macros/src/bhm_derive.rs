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
