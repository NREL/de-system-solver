use crate::imports::*;

pub fn solver_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let ident = &ast.ident;

    let mut impl_block = TokenStream2::default();

    impl_block.extend::<TokenStream2>(quote! {
        impl #ident {
            pub fn walk(&mut self, solver_opts: SolverOptions, end_time: f64) {
                match solver_opts {
                    SolverOptions::FixedEuler { dt } => {
                        while self.state.time < end_time {
                            self.step(dt)
                        }
                    }
                    _ => todo!(),
                }
            }
        }
    });
    impl_block.into()
}
