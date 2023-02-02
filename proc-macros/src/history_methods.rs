use crate::imports::*;

pub(crate) fn history_methods_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let ident = &ast.ident;

    let fields: Vec<Field> = match ast.data {
        syn::Data::Struct(s) => s.fields.iter().map(|x| x.clone()).collect(),
        _ => panic!("only works on structs"),
    };

    let struct_has_state = fields
        .iter()
        .any(|x| x.ident.as_ref().unwrap().to_string() == "state");

    let has_state_vec: Vec<bool> = fields
        .iter()
        .map(|field| {
            field
                .attrs
                .iter()
                .any(|attr| attr.path.is_ident("has_state") || attr.path.is_ident("history"))
        })
        .collect();

    let fields_with_state = fields
        .iter()
        .zip(has_state_vec.clone())
        .filter(|(_f, hsv)| *hsv)
        .map(|(f, _hsv)| f.ident.as_ref().unwrap())
        .collect::<Vec<_>>();

    let mut impl_block = TokenStream2::default();

    if struct_has_state || !fields_with_state.is_empty() {
        // struct has state and has fields with state
        let save_state_cust_doc: TokenStream2 = format!(
            "/// Saves `self.state` to `self.history` and propagtes to `save_state` in {}",
            fields_with_state
                .iter()
                .map(|x| format!("[Self::{}]", x.to_string()))
                .collect::<Vec<String>>()
                .join(", ")
        )
        .parse()
        .unwrap();

        let self_state_push: TokenStream2 = if struct_has_state {
            quote! {self.history.push(self.state)}
        } else {
            quote! {}
        };

        impl_block.extend::<TokenStream2>(quote! {
            impl #ident {
                #save_state_cust_doc
                pub fn save_state(&mut self) {
                    #self_state_push;
                    #(self.#fields_with_state.save_state();)*
                }
            }
        });
    } else {
        // struct does not have state and has no fields with state
        panic!("HistoryMethods does not work here.");
    }
    impl_block.into()
}
