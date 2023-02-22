use crate::imports::*;

#[common_derives]
pub enum SolverOptions {
    /// Euler with fixed time step
    #[default]
    EulerFixed,
    RK4Fixed,
    // TODO: add this stuff back into fixed options
    // /// time step to use if `t_report` is larger than `dt`
    // dt: f64,
    /// Runge-Kutta 4/5 order adaptive, Cash-Karp method
    /// https://en.wikipedia.org/wiki/Cash%E2%80%93Karp_method
    RK45CashKarp(AdaptiveSolver),
    ToDo,
}

#[pyo3_api]
#[common_derives]
#[derive(Builder)]
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

#[common_derives]
#[pyo3_api]
pub struct SolverHistory {
    /// number of iterations to achieve tolerance
    n_iters: u8,
    /// L2 (euclidean) norm
    norm: f64,
    // TODO: put more things here
    // - vec of time steps taken between elements of `t_report`
    // - vec of vec of states at each time step
}
