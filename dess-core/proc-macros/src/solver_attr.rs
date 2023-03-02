use crate::imports::*;

/// Derives several methods for struct
pub(crate) fn solver_attr(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast_item = item.clone();
    let ast = syn::parse_macro_input!(ast_item as syn::DeriveInput);
    let ident = &ast.ident;

    let fields: Vec<Field> = match ast.data {
        syn::Data::Struct(s) => s.fields.iter().map(|x| x.clone()).collect(),
        _ => abort!(&ident.span(), "only works on structs"),
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

    let mut item_and_impl_block = TokenStream2::default();

    item_and_impl_block.extend::<TokenStream2>(item.clone().into());

    let attr: TokenStream2 = attr.into();

    item_and_impl_block.extend::<TokenStream2>(quote! {
        impl SolverBase for #ident {
            /// assuming `set_derivs` has been called, steps
            /// value of states by deriv * dt
            fn step_by_dt(&mut self, dt: &f64) {
                #(self.#fields_with_state.step_state_by_dt(dt);)*
            }

            /// assuming `set_derivs` has been called, steps
            /// value of states by deriv * dt
            fn step(&mut self, val: Vec<f64>) {
                let mut iter = val.iter();
                #(self.#fields_with_state.step_state(iter.next().unwrap().clone());)*
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
            fn set_derivs(&mut self, val: &Vec<f64>) {
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

            #attr
        }

        impl SolverVariantMethods for #ident{}

        impl #ident {
            /// iterates through time until last value of `t_report`
            pub fn walk(&mut self) {
                while &self.state.time < self.t_report.last().unwrap() {
                    self.solve_step();
                    self.state.i += 1;
                    self.save_state();
                }
            }

            /// Runs `solver_type` specific step method that calls
            /// [Self::step] in solver-specific manner
            pub fn solve_step(&mut self) {
                while self.state.time < self.t_report[self.state.i] {
                    let dt = self.t_report[self.state.i] - self.state.time;
                    match &self.solver_type {
                        SolverTypes::EulerFixed{dt: dt_fixed} => {
                            let dt = dt.min(dt_fixed.clone());
                            self.euler(&dt);
                            self.state.time += dt;
                        },
                        SolverTypes::RK4Fixed{dt: dt_fixed} => {
                            let dt = dt.min(dt_fixed.clone());
                            self.rk4fixed(&dt);
                            self.state.time += dt;
                        },
                        SolverTypes::RK45CashKarp(_sc) => {
                            let dt = self.rk45_cash_karp(&dt);
                            self.state.time += dt;
                        },
                        _ => todo!(),
                    }
                }
            }

            /// solves time step with adaptive Cash-Karp Method (variant of RK45) and returns `dt` used
            /// https://en.wikipedia.org/wiki/Cash%E2%80%93Karp_method
            fn rk45_cash_karp(&mut self, dt_max: &f64) -> f64 {
                let sc = match &self.solver_type {
                    SolverTypes::RK45CashKarp(sc) => sc,
                    _ => unreachable!(),
                };
                let mut dt = dt_max.min(sc.state.dt_prev);

                let delta5 = loop {
                    let (delta4, delta5) = self.rk45_cash_karp_step(dt);
                    let sc = match &mut self.solver_type {
                        SolverTypes::RK45CashKarp(sc) => sc,
                        _ => unreachable!(),
                    };
                    sc.state.norm = delta4
                        .iter()
                        .zip(&delta5)
                        .map(|(d4, d5)| d4.powi(2) - d5.powi(2))
                        .collect::<Vec<f64>>()
                        .iter()
                        .sum::<f64>()
                        .sqrt();
                    sc.state.n_iter += 1;

                    /// TODO: think about the epsilon arg (3rd arg) here:
                    let tol_met: bool = almost_eq(sc.state.norm, sc.tol, Some(1e-3));

                    if tol_met || sc.state.n_iter >= sc.max_iter {
                        sc.state.dt_prev = dt;
                        sc.state.t_curr = self.state.time;
                        break delta5
                    };

                    let dt_coeff = (sc.tol / sc.state.norm).abs().powf(
                        if sc.state.norm < sc.tol {
                            0.2
                        } else {
                            0.25
                        }
                    );
                    dt *= dt_coeff;
                };

                // increment forward with 5th order solution
                self.step(delta5);

                dt
            }
        }
    });

    item_and_impl_block.into()
}
