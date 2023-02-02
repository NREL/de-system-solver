mod imports;
use imports::*;
mod history_methods;
mod history_vec;
mod solver_derive;
mod utilities;

/// generate HistoryVec that acts like a vec of States but
/// stores each field of state as a vec field.
#[proc_macro_derive(HistoryVec)]
pub fn history_vec_derive(input: TokenStream) -> TokenStream {
    history_vec::history_vec_derive(input)
}

#[proc_macro_error]
#[proc_macro_derive(HistoryMethods, attributes(has_state, history))]
pub fn history_methods_derive(input: TokenStream) -> TokenStream {
    history_methods::history_methods_derive(input)
}

#[proc_macro_error]
#[proc_macro_derive(Solver)]
pub fn solver_derive(input: TokenStream) -> TokenStream {
    solver_derive::solver_derive(input)
}

#[proc_macro_error]
#[proc_macro_derive(GetStateValues)]
pub fn get_state_vals_derive(input: TokenStream) -> TokenStream {
    solver_derive::get_state_vals_derive(input)
}
