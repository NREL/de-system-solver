mod imports;
use imports::*;
mod bare_clone;
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

/// Derives `save_state` method for struct and all fields marked with
/// `history` or `has_state` attributes
#[proc_macro_error]
#[proc_macro_derive(HistoryMethods, attributes(has_state, history))]
pub fn history_methods_derive(input: TokenStream) -> TokenStream {
    history_methods::history_methods_derive(input)
}

/// Derives `walk` method for struct
#[proc_macro_error]
#[proc_macro_derive(Walk)]
pub fn walk_derive(input: TokenStream) -> TokenStream {
    solver_derive::walk_derive(input)
}

/// Derives `get_state_values` method for struct with fields marked with
/// `has_state` attribute
#[proc_macro_error]
#[proc_macro_derive(GetStateValues, attributes(has_state))]
pub fn get_state_vals_derive(input: TokenStream) -> TokenStream {
    solver_derive::get_state_vals_derive(input)
}

/// Derives `get_state_values` method for struct with fields marked with
/// `has_state` attribute
#[proc_macro_error]
#[proc_macro_derive(BareClone, attributes(has_state, history))]
pub fn bare_clone(input: TokenStream) -> TokenStream {
    bare_clone::bare_clone_derive(input)
}

// TODO: make an attribute-style macro that creates:
// - pyo3 api
// - serde api (e.g. to/from_file, json, yaml, bincode)
