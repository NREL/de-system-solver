use crate::imports::*;

#[common_derives]
pub enum SolverOptions {
    /// Euler with fixed time step.
    /// parameter `dt` provides time step size for whenever solver is between
    /// `t_report` times.  
    EulerFixed {
        dt: f64,
    },
    /// Runge-Kutta 4th order with fixed time step.  
    /// parameter `dt` provides time step size for whenever solver is between
    /// `t_report` times.  
    RK4Fixed {
        dt: f64,
    },
    // TODO: add this stuff back into fixed options
    // /// time step to use if `t_report` is larger than `dt`
    // dt: f64,
    /// Runge-Kutta 4/5 order adaptive, Cash-Karp method
    /// https://en.wikipedia.org/wiki/Cash%E2%80%93Karp_method
    RK45CashKarp(AdaptiveSolver),
    ToDo,
}

impl Default for SolverOptions {
    fn default() -> Self {
        SolverOptions::RK4Fixed { dt: 0.1 }
    }
}

#[pyo3_api(
    #[new]
    fn new_py(
        dt_max: f64,
        max_iter: u32,
        tol: f64,
        save: bool,
    ) -> Self {
        Self::new(dt_max, max_iter, tol, save)
    }
)]
#[common_derives]
#[derive(HistoryMethods)]
pub struct AdaptiveSolver {
    /// max allowable dt
    pub dt_max: f64,
    /// max number of iterations per time step
    pub max_iter: u32,
    /// euclidean error tolerance
    pub tol: f64,
    /// save iteration history
    pub save: bool,
    /// current iteration variables
    pub state: SolverState,
    /// iteration history
    pub history: SolverStateHistoryVec,
}

impl AdaptiveSolver {
    pub fn new(dt_max: f64, max_iter: u32, tol: f64, save: bool) -> Self {
        Self {
            dt_max,
            max_iter,
            tol,
            save,
            ..Default::default()
        }
    }
}

impl Default for AdaptiveSolver {
    fn default() -> Self {
        Self {
            dt_max: 1.0,
            max_iter: 2,
            tol: 1e-6,
            save: false,
            state: Default::default(),
            history: Default::default(),
        }
    }
}

#[common_derives]
#[pyo3_api]
#[derive(HistoryVec, Copy)]
pub struct SolverState {
    /// time step size in previous interval
    pub dt_prev: f64,
    /// number of iterations to achieve tolerance
    pub n_iters: u8,
    /// L2 (euclidean) norm
    pub norm: f64,
    /// current system time used in solver
    pub t_curr: f64,
}

impl Default for SolverState {
    fn default() -> Self {
        Self {
            dt_prev: 0.1,
            n_iters: 0,
            norm: 0.0,
            t_curr: 0.0,
        }
    }
}
