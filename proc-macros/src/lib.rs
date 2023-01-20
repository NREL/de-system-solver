mod imports;
use imports::*;
mod bhm_derive;
mod history_vec_derive;
mod utilities;

#[proc_macro_derive(HistoryVec)]
/// generate HistoryVec that acts like a vec of States but
/// stores each field of state as a vec field.
pub fn history_vec_derive(input: TokenStream) -> TokenStream {
    history_vec_derive::history_vec_derive(input)
}

#[proc_macro_error]
#[proc_macro_derive(BasicHistoryMethods)]
pub fn basic_history_methods_derive(input: TokenStream) -> TokenStream {
    bhm_derive::basic_history_methods_derive(input)
}
