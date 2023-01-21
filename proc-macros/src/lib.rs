mod imports;
use imports::*;
mod history_methods;
mod history_vec;
mod utilities;

#[proc_macro_derive(HistoryVec)]
/// generate HistoryVec that acts like a vec of States but
/// stores each field of state as a vec field.
pub fn history_vec_derive(input: TokenStream) -> TokenStream {
    history_vec::history_vec_derive(input)
}

#[proc_macro_error]
#[proc_macro_derive(BasicHistoryMethods)]
pub fn basic_history_methods_derive(input: TokenStream) -> TokenStream {
    history_methods::basic_history_methods_derive(input)
}

#[proc_macro_error]
#[proc_macro_derive(NestedHistoryMethods, attributes(has_state))]
pub fn nested_history_methods_derive(input: TokenStream) -> TokenStream {
    history_methods::nested_history_methods_derive(input)
}
