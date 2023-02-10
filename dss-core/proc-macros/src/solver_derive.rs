use crate::imports::*;

/// Derives `walk` method for struct
pub(crate) fn walk_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let ident = &ast.ident;

    let mut impl_block = TokenStream2::default();

    impl_block.extend::<TokenStream2>(quote! {
        impl #ident {
            pub fn walk(&mut self, solver_opts: SolverOptions, end_time: f64) {
                self.save_state();
                    while self.state.time < end_time {
                        self.solve_step(&solver_opts);
                    }
                }

            /// Runs `solver_opts` specific step method that calls
            /// [Self::step] in solver-specific manner
            pub fn solve_step(&mut self, solver_opts: &SolverOptions) {
                match solver_opts {
                    SolverOptions::FixedEuler { dt } => {
                        self.step(&dt);
                        self.state.time += dt;
                    },
                    // SolverOptions::RK3Adaptive(rk3a):: => {
                        //     while self.state.time < end_time {
                            //         self.step(dt)
                            //     }
                            // },
                    _ => todo!(),
                }
                self.save_state();
            }
        }
    });
    impl_block.into()
}

/// Derives `get_state_values` method for struct with fields marked with
/// `has_state` attribute
pub(crate) fn get_state_vals_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let ident = &ast.ident;

    let fields: Vec<Field> = match ast.data {
        syn::Data::Struct(s) => s.fields.iter().map(|x| x.clone()).collect(),
        _ => panic!("only works on structs"),
    };

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
        .zip(has_state_vec.clone())
        .filter(|(_f, hsv)| *hsv)
        .map(|(f, _hsv)| f.ident.as_ref().unwrap())
        .collect::<Vec<_>>();

    let mut impl_block = TokenStream2::default();

    impl_block.extend::<TokenStream2>(quote! {
        impl #ident {
            pub fn get_state_vals(&self) -> Vec<f64> {
                let mut state_vec: Vec<f64> = vec![];
                #(state_vec.push(self.#fields_with_state.state.pot());)*
                state_vec
            }
        }
    });

    impl_block.into()
}
