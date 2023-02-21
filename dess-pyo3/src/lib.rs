use dess_core::prelude::*;
use dess_examples::*;
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn dess_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<AdaptiveSolver>()?;
    m.add_class::<SolverHistory>()?;
    Ok(())
}
