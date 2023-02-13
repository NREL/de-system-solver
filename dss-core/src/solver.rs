pub enum SolverOptions {
    /// Euler with fixed time step
    FixedEuler {
        dt: f64,
    },
    /// Runge-Kutta, 3rd order, adaptive time step
    RK3Adaptive(RK3Adaptive),
    ToDo,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]

pub struct RK3Adaptive {
    /// max allowable dt
    pub dt_max: f64,
    /// max number of iterations per time step
    pub max_iter: f64,
    /// euclidean error tolerance
    pub tol: f64,
    /// save iteration history
    pub save: bool,
    /// time step size in previous interval
    pub dt_prev: f64,
    /// iteration history
    pub history: Vec<SolverHistory>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]

/// Reasons for stop of iteration
pub enum SolverReasons {
    /// iterations stopped because error tolerance was achieved
    ToleranceAchieved,
    /// iterations stopped because max_iter was achieved
    MaxIter,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]

pub struct SolverHistory {
    /// number of iterations to achieve tolerance
    n_iters: u8,
    /// reason iteration stopped
    reason: SolverReasons,
    /// L2 (euclidean) norm
    norm: f64,
}

pub fn rk4fixed(
    sys: Box<dyn crate::traits_and_macros::GetStateValues>,
) -> (Box<dyn crate::traits_and_macros::GetStateValues>, Vec<f64>) {
    (sys, vec![0.4, 0.5, 0.6])
}
