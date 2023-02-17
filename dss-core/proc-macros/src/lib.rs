mod imports;
use imports::*;
mod bare_clone;
mod history_methods;
mod history_vec;
mod pyo3_api;
mod solver_attr;
mod utilities;

/// generate HistoryVec that acts like a vec of States but
/// stores each field of state as a vec field.
#[proc_macro_error]
#[proc_macro_derive(HistoryVec)]
pub fn history_vec_derive(input: TokenStream) -> TokenStream {
    history_vec::history_vec_derive(input)
}

/// Derives `save_state` method for struct and all fields marked with
/// `save_state` or `use_state` attributes
#[proc_macro_error]
#[proc_macro_derive(HistoryMethods, attributes(use_state, save_state))]
pub fn history_methods_derive(input: TokenStream) -> TokenStream {
    history_methods::history_methods_derive(input)
}

/// Generates several methods for struct to create solver framework
#[proc_macro_error]
#[proc_macro_attribute]
pub fn solver(attr: TokenStream, input: TokenStream) -> TokenStream {
    solver_attr::solver_attr(attr, input)
}

/// Derives `bare_clone` method
#[proc_macro_error]
#[proc_macro_derive(BareClone, attributes(use_state, save_state))]
pub fn bare_clone(input: TokenStream) -> TokenStream {
    bare_clone::bare_clone_derive(input)
}

// TODO: make an attribute-style macro that creates:
/// Generates pyo3 api for struct
#[proc_macro_error]
#[proc_macro_attribute]
pub fn pyo3_api(attr: TokenStream, input: TokenStream) -> TokenStream {
    pyo3_api::pyo3_api(attr, input)
}
