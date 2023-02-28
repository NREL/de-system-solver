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

            /// Runs `solver_opts` specific step method that calls
            /// [Self::step] in solver-specific manner
            pub fn solve_step(&mut self) {
                while self.state.time < self.t_report[self.state.i] {
                    let dt = match self.solver_opts {
                        SolverOptions::EulerFixed{dt} => {
                            let dt = (self.t_report[self.state.i] - self.state.time).min(dt);
                            self.euler(&dt);
                            dt
                        },
                        SolverOptions::RK4Fixed{dt} => {
                            let dt = (self.t_report[self.state.i] - self.state.time).min(dt);
                            self.rk4fixed(&dt);
                            dt
                        },
                        // SolverOptions::RK45CashKarp(solver) => {
                        //     let dt = self.t_report[self.state.i] - self.state.time;
                        //     self.rk45_cash_karp(&dt, solver)
                        // },
                        _ => todo!(),
                    };
                    self.state.time += dt;
                }
            }
        }
    });

    item_and_impl_block.into()
}
