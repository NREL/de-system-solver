use crate::imports::*;

/// Derives several methods for struct
pub(crate) fn walk_derive(input: TokenStream) -> TokenStream {
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
        impl dss_core::prelude::Walk for #ident {
            fn walk(&mut self, end_time: f64) {
                self.save_state();
                    while self.state.time < end_time {
                        self.solve_step();
                    }
                }

            /// Runs `solver_opts` specific step method that calls
            /// [Self::step] in solver-specific manner
            fn solve_step(&mut self) {
                match self.solver_opts {
                    SolverOptions::FixedEuler { dt } => {
                        self.step(&dt);
                        self.state.time += dt;
                    },
                    // SolverOptions::RK3Adaptive(rk3a) => {
                    //     // initial guess for time step size
                    //     let h = rk3a.dt_prev;
                    //     let k1: Vec<f64> = self.get_state_vals();
                    //     let k2: Vec<f64> =
                    //     let rk4states = 666.6;
                    //     let rk5states = 666.6;
                    // },
                    _ => todo!(),
                }
                self.save_state();
            }

            /// assuming `set_derivs` or `step_derivs` has been called, steps
            /// value of states by deriv * dt
            fn step_states(&mut self, dt: &f64) {
                #(self.#fields_with_state.step_state(dt);)*
            }

            /// reset all time derivatives to zero for start of `solve_step`
            fn reset_derivs(&mut self) {
                #(self.#fields_with_state.set_deriv(0.0);)*
            }

            fn get_derivs(&self) -> Vec<f64> {
                let mut derivs: Vec<f64> = Vec::new();
                #(derivs.push(self.#fields_with_state.deriv());)*
                derivs
            }

            fn get_states(&self) -> Vec<f64> {
                let mut states: Vec<f64> = Vec::new();
                #(states.push(self.#fields_with_state.state());)*
                states
            }
        }
    });

    impl_block.into()
}
