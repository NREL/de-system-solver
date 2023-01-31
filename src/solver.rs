pub enum Solver {
    FixedEuler {
        dt: f64,
    },
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
