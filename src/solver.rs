pub enum Solver {
    /// Euler with fixed time step
    FixedEuler {
        dt: f64,
    },
    /// Runga-Kutta, 3rd order, adaptive time step
    RK3Adaptive {
        /// max allowable dt
        dt_max: f64,
        /// max number of iterations per time step
        max_iter: f64,
        /// absolute convergence tolerance
        atol: f64,
        /// relative convergence tolerance
        rtol: f64,
    },
    ToDo,
}
