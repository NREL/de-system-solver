mod imports;
pub mod prelude;
#[cfg(feature = "pyo3")]
pub(crate) mod pyo3_imports;
mod solver;
pub mod traits_and_macros;
pub mod utilities;

use prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, HistoryVec, Default)]
#[common_derives]
#[pyo3_api]
pub struct SystemState {
    // current index in `t_report`
    pub i: usize,
    // current time
    pub time: f64,
}
