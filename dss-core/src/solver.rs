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
    dt_max: f64,
    /// max number of iterations per time step
    max_iter: f64,
    /// euclidean error tolerance
    tol: f64,
    /// save iteration history
    save: bool,
    /// time step size in previous interval
    dt_prev: f64,
    /// iteration history
    history: Vec<SolverHistory>,
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
