use crate::imports::*;

/// Derives several methods for struct
pub(crate) fn solver_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let ident = &ast.ident;

    let fields: Vec<Field> = match ast.data {
        syn::Data::Struct(s) => s.fields.iter().map(|x| x.clone()).collect(),
        _ => panic!("only works on structs"),
    };

    let use_state_vec: Vec<bool> = fields
        .iter()
        .map(|field| {
            field
                .attrs
                .iter()
                .any(|attr| attr.path.is_ident("use_state"))
        })
        .collect();

    let fields_with_state = fields
        .iter()
        .zip(use_state_vec.clone())
        .filter(|(_f, hsv)| *hsv)
        .map(|(f, _hsv)| f.ident.as_ref().unwrap())
        .collect::<Vec<_>>();

    let mut impl_block = TokenStream2::default();

    impl_block.extend::<TokenStream2>(quote! {
        impl #ident {
            /// iterates through time until last value of `t_report`
            fn walk(&mut self) {
                self.save_state();
                    while &self.state.time < self.t_report.last().unwrap() {
                        self.solve_step();
                    }
                }

            /// Runs `solver_opts` specific step method that calls
            /// [Self::step] in solver-specific manner
            fn solve_step(&mut self) {
                let dt = match self.solver_opts {
                    SolverOptions::EulerFixed => {
                        let dt = self.t_report[self.state.i] - self.state.time;
                        self.step(&dt);
                        dt
                    },
                    // SolverOptions::RK4Fixed => {
                    //     let (dt, _ks) = self.rk4fixed();
                    //     dt
                    // }
                    _ => todo!(),
                };
                self.state.time += dt;
                self.state.i += 1;
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

            /// returns derivatives of states
            fn get_derivs(&self) -> Vec<f64> {
                let mut derivs: Vec<f64> = Vec::new();
                #(derivs.push(self.#fields_with_state.deriv());)*
                derivs
            }

            /// returns values of states
            fn get_states(&self) -> Vec<f64> {
                let mut states: Vec<f64> = Vec::new();
                #(states.push(self.#fields_with_state.state());)*
                states
            }

            // /// solves time step with 4th order fixed-step Runge-Kutta
            // /// method and returns k-values
            // fn rk4fixed(&mut self) -> (dt, Vec<Vec<f64>) {
            //     let dt = self.t_report[self.state.i] - self.state.time;
            //     self.update_derivs();
            //     let k1 = self.get_derivs();
            //     (dt, vec![k1])
            // }
        }
    });

    impl_block.into()
}
