use dess_core::prelude::*;
use dess_examples::*;

/// A Python module implemented in Rust.
#[pymodule]
fn dess_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<AdaptiveSolverConfig>()?;
    m.add_class::<SolverState>()?;
    m.add_class::<SolverStateHistoryVec>()?;
    m.add_class::<System>()?;
    m.add_class::<SystemState>()?;
    m.add_class::<SystemStateHistoryVec>()?;
    m.add_class::<ThermalMass>()?;
    m.add_class::<ThermalMassState>()?;
    m.add_class::<ThermalMassStateHistoryVec>()?;
    m.add_class::<Conductance>()?;
    m.add_class::<ConductanceState>()?;
    m.add_class::<ConductanceStateHistoryVec>()?;
    Ok(())
}
