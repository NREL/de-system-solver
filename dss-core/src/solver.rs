use crate::imports::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum SolverOptions {
    /// Euler with fixed time step
    EulerFixed,
    // TODO: add this stuff back into fixed options
    // /// time step to use if `t_report` is larger than `dt`
    // dt: f64,
    RK4Fixed,
    RK5Fixed,
    /// Runge-Kutta, 3rd order, adaptive time step
    RK45Adaptive(AdaptiveSolver),
    ToDo,
}

impl Default for SolverOptions {
    fn default() -> Self {
        SolverOptions::EulerFixed
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct AdaptiveSolver {
    /// max allowable dt
    pub dt_max: f64,
    /// max number of iterations per time step
    pub max_iter: f64,
    /// euclidean error tolerance
    pub tol: f64,
    /// time step size in previous interval
    pub dt_prev: f64,
    /// save iteration history
    pub save: bool,
    /// iteration history
    pub history: Vec<SolverHistory>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]

pub struct SolverHistory {
    /// number of iterations to achieve tolerance
    n_iters: u8,
    /// L2 (euclidean) norm
    norm: f64,
}
