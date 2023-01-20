mod imports;
use imports::*;
mod history_vec_derive;
mod utilities;

#[proc_macro_derive(HistoryVec)]
/// generate HistoryVec that acts like a vec of States but
/// stores each field of state as a vec field.
pub fn history_vec_derive(input: TokenStream) -> TokenStream {
    history_vec_derive::history_vec_derive(input)
}
