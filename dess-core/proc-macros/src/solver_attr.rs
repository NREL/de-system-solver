use crate::imports::*;
use crate::utilities::parse_ts_as_fn_defs;

/// Derives several methods for struct
pub(crate) fn solver_attr(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_struct = syn::parse_macro_input!(item as ItemStruct);
    let ident = &item_struct.ident;

    let expected_exclusive = true;
    let expected_fn_names = Vec::<String>::from(["update_derivs".into()]);
    let forbidden_fn_names = Vec::<String>::new();

    let fn_from_attr = parse_ts_as_fn_defs(
        attr,
        expected_fn_names,
        expected_exclusive,
        forbidden_fn_names,
    );

    let fields = &item_struct.fields;
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
        .zip(use_state_vec)
        .filter(|(_f, hsv)| *hsv)
        .map(|(f, _hsv)| f.ident.as_ref().unwrap())
        .collect::<Vec<_>>();

    let mut item_and_impl_block = TokenStream2::default();

    item_and_impl_block.extend::<TokenStream2>(item_struct.to_token_stream());
    item_and_impl_block.extend::<TokenStream2>(quote! {
        impl HasStates for #ident {
            /// returns values of states
            fn states(&self) -> Vec<f64> {
                let mut states: Vec<f64> = Vec::new();
                #(states.push(self.#fields_with_state.state());)*
                states
            }
            /// sets values of states
            fn set_states(&mut self, val: Vec<f64>) {
                let mut iter = val.iter();
                #(self.#fields_with_state.set_state(iter.next().unwrap().clone());)*
            }
            /// assuming `set_derivs` has been called, steps
            /// value of states by deriv * dt
            fn step_states_by_dt(&mut self, dt: &f64) {
                #(self.#fields_with_state.step_state_by_dt(dt);)*
                self.step_time(dt);
            }
            /// assuming `set_derivs` has been called, steps
            /// value of states by deriv * dt
            fn step_states(&mut self, val: Vec<f64>) {
                let mut iter = val.iter();
                #(self.#fields_with_state.step_state(iter.next().unwrap().clone());)*
            }
            /// returns derivatives of states
            fn derivs(&self) -> Vec<f64> {
                let mut derivs: Vec<f64> = Vec::new();
                #(derivs.push(self.#fields_with_state.deriv());)*
                derivs
            }
            /// sets values of derivatives of states
            fn set_derivs(&mut self, val: &[f64]) {
                let mut iter = val.iter();
                #(self.#fields_with_state.set_deriv(iter.next().unwrap().clone());)*
            }
            /// steps derivs by val
            fn step_derivs(&mut self, val: Vec<f64>) {
                let mut iter = val.iter();
                #(self.#fields_with_state.step_deriv(iter.next().unwrap().clone());)*
            }
            /// returns value of storage variable (e.g. thermal capacitance \[J/K\])
            fn storages(&self) -> Vec<f64> {
                let mut storages: Vec<f64> = Vec::new();
                #(storages.push(self.#fields_with_state.storage());)*
                storages
            }
        }

        impl SolverBase for #ident {
            /// reset all time derivatives to zero for start of `solve_step`
            fn reset_derivs(&mut self) {
                #(self.#fields_with_state.set_deriv(0.0);)*
            }
            /// steps dt without affecting states
            fn step_time(&mut self, dt: &f64) {
                self.state.time += dt;
            }
            fn sc(&self) -> Option<&AdaptiveSolverConfig> {
                match &self.solver_type {
                    SolverTypes::RK45CashKarp(sc) => Some(sc),
                    SolverTypes::RK23BogackiShampine(sc) => Some(sc),
                    _ => None,
                }
            }
            fn sc_mut(&mut self) -> Option<&mut AdaptiveSolverConfig> {
                match &mut self.solver_type {
                    SolverTypes::RK45CashKarp(sc) => Some(sc),
                    SolverTypes::RK23BogackiShampine(sc) => Some(sc),
                    _ => None,
                }
            }
            fn state(&self) -> &dess_core::SystemState {
                &self.state
            }
            #fn_from_attr
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
                        },
                        SolverTypes::HeunsMethod{dt: dt_fixed} => {
                            let dt = dt.min(dt_fixed.clone());
                            self.heun(&dt);
                        },
                        SolverTypes::MidpointMethod{dt: dt_fixed} => {
                            let dt = dt.min(dt_fixed.clone());
                            self.midpoint(&dt);
                        },
                        SolverTypes::RalstonsMethod{dt: dt_fixed} => {
                            let dt = dt.min(dt_fixed.clone());
                            self.ralston(&dt);
                        },
                        SolverTypes::RK23BogackiShampine(_sc) => {
                            let dt = self.rk23_bogacki_shampine(&dt);
                        },
                        SolverTypes::RK4Fixed{dt: dt_fixed} => {
                            let dt = dt.min(dt_fixed.clone());
                            self.rk4fixed(&dt);
                        },
                        SolverTypes::RK45CashKarp(_sc) => {
                            let dt = self.rk45_cash_karp(&dt);
                        },
                    }
                }
            }
        }
    });
    item_and_impl_block.into()
}
