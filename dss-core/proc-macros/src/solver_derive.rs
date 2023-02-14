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

            /// sets values of derivatives of states
            fn set_derivs(&mut self, val: Vec<f64>) {
                let mut iter = val.iter();
                #(self.#fields_with_state.set_deriv(iter.next().unwrap().clone());)*
            }

            /// returns values of states
            fn get_states(&self) -> Vec<f64> {
                let mut states: Vec<f64> = Vec::new();
                #(states.push(self.#fields_with_state.state());)*
                states
            }

            /// sets values of states
            fn set_states(&mut self, val: Vec<f64>) {
                let mut iter = val.iter();
                #(self.#fields_with_state.set_state(iter.next().unwrap().clone());)*
            }

            /// solves time step with 4th order Runge-Kutta method
            fn rk4fixed(&mut self) {
                let dt = self.t_report[self.state.i] - self.state.time;
                self.update_derivs();
                // k1 = f(x_i, y_i)
                let k1 = self.get_derivs();
                // k2 = f(x_i + 1 / 2 * h, y_i + 1 / 2 * k1 * h)
                let mut sys1 = self.bare_clone();
                sys1.step(&(dt / 2.0));
                sys1.update_derivs();
                let k2 = sys1.get_derivs();

                // k3 = f(x_i + 3 / 10 * h, y_i + 3 / 40 * k1 * h + 9 / 40 * k2 * h)

                // k4 = f(x_i + 3 / 5 * h, y_i + 3 / 10 * k1 * h - 9 / 10 * k2 * h + 6 / 5 * k3 * h)

                // k5 = f(x_i + h, y_i - 11 / 54 * k1 * h + 5 / 2 * k2 * h - 70 / 27 * k3 * h + 35 / 27 * k4 * h)

                // k6 = f(x_i + 7 / 8 * h, y_i + 1631 / 55296 * k1 * h + 175 / 512 * k2 * h + 575 / 13824 * k3 * h + 44275 / 110592 * k4 * h + 253 / 4096 * k4 * h)
            }

            // /// solves time step with adaptive Cash-Karp Method (variant of RK45)
            // /// https://en.wikipedia.org/wiki/Cash%E2%80%93Karp_method
            // fn rk45CashKarp(&mut self) {
            //     let dt = self.t_report[self.state.i] - self.state.time;
            //     self.update_derivs();
            //     // k1 = f(x_i, y_i)
            //     let k1 = self.get_derivs();
            //     // k2 = f(x_i + 1 / 5 * h, y_i + 1 / 5 * k1 * h)
            //     let k2 = self.get_states().iter().zip(k1).map(|(x, k)| *x + k * dt);

            //     // k3 = f(x_i + 3 / 10 * h, y_i + 3 / 40 * k1 * h + 9 / 40 * k2 * h)

            //     // k4 = f(x_i + 3 / 5 * h, y_i + 3 / 10 * k1 * h - 9 / 10 * k2 * h + 6 / 5 * k3 * h)

            //     // k5 = f(x_i + h, y_i - 11 / 54 * k1 * h + 5 / 2 * k2 * h - 70 / 27 * k3 * h + 35 / 27 * k4 * h)

            //     // k6 = f(x_i + 7 / 8 * h, y_i + 1631 / 55296 * k1 * h + 175 / 512 * k2 * h + 575 / 13824 * k3 * h + 44275 / 110592 * k4 * h + 253 / 4096 * k4 * h)
            // }
        }
    });

    impl_block.into()
}
