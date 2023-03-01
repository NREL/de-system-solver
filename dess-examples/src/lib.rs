pub mod imports;
pub use imports::*;
pub mod components;
pub use components::*;
mod tests;

/// System of connected components
#[derive(HistoryMethods, BareClone)]
#[pyo3_api(
    #[new]
    fn __new__(
        solver_conf: String,
        m1: ThermalMass,
        m2: ThermalMass,
        h12: Conductance,
        m3: ThermalMass,
        h13: Conductance,
        t_report: Vec<f64>,
    ) -> Self {
        Self::new(
            SolverOptions::from_json(&solver_conf).unwrap(),
            m1,
            m2,
            h12,
            m3,
            h13,
            t_report,
        )
    }

    #[classmethod]
    fn new_rk45_cash_karp(
        _cls: &PyType, 
        sol: AdaptiveSolverConfig,
        m1: ThermalMass,
        m2: ThermalMass,
        h12: Conductance,
        m3: ThermalMass,
        h13: Conductance,
        t_report: Vec<f64>,
    ) -> Self {
        Self::new(
            SolverOptions::RK45CashKarp(sol),
            m1,
            m2,
            h12,
            m3,
            h13,
            t_report
        )
    }

    #[getter]
    fn get_solver_conf(&self) -> String {
        self.solver_conf.to_json()
    }

    #[getter]
    fn get_rk45_solver_conf(&self) -> Option<AdaptiveSolverConfig> {
        if let SolverOptions::RK45CashKarp(solver_conf) = &self.solver_conf {
            Some(solver_conf.clone())
        } else {
            None
        }
    }

    #[pyo3(name = "walk")]
    fn walk_py(&mut self) {
        self.walk();
    }
)]
#[solver(
    /// Updates time derivatives of states.
    /// This method must be user defined in `solver` macro args.
    fn update_derivs(&mut self) {
        self.reset_derivs();
        connect_states!(self, (m1, m2, h12), (m1, m3, h13));
        update_derivs!(self, (m1, m2, h12), (m1, m3, h13));
    }
)]
#[common_derives]
#[derive(Default)]
pub struct System {
    #[skip_get]
    solver_conf: SolverOptions,
    // components
    // the `use_state` attribute tells the SystemSolver
    #[use_state]
    pub m1: ThermalMass,
    #[use_state]
    pub m2: ThermalMass,
    /// h12 connects m1 to m2
    #[save_state]
    pub h12: Conductance,
    #[use_state]
    pub m3: ThermalMass,
    #[save_state]
    pub h13: Conductance,
    pub t_report: Vec<f64>,

    pub state: SystemState,
    pub history: SystemStateHistoryVec,
}

impl System {
    pub fn new(
        solver_conf: SolverOptions,
        m1: ThermalMass,
        m2: ThermalMass,
        h12: Conductance,
        m3: ThermalMass,
        h13: Conductance,
        t_report: Vec<f64>,
    ) -> Self {
        Self {
            solver_conf,
            m1,
            m2,
            h12,
            m3,
            h13,
            t_report,
            state: Default::default(),
            history: Default::default(),
        }
    }
}

#[derive(Copy, HistoryVec, Default)]
#[common_derives]
#[pyo3_api]
pub struct SystemState {
    // current index in `t_report`
    i: usize,
    // current time
    time: f64,
}

#[macro_export]
macro_rules! time_it {
    ($thing: expr) => {{
        let t0 = Instant::now();
        $thing;
        let t_elapsed = Instant::now() - t0;
        t_elapsed
    }};
}

pub fn mock_euler_sys() -> System {
    let m1 = ThermalMass::new(1.0, 0.0);
    let m2 = ThermalMass::new(2.0, 10.0);
    let h12 = Conductance::new(5.0);
    let m3 = ThermalMass::new(1.5, 12.0);
    let h13 = Conductance::new(5.0);
    let t_report: Vec<f64> = Vec::linspace(0.0, 1.0, 201);

    System::new(
        SolverOptions::EulerFixed { dt: 5e-3 },
        m1,
        m2,
        h12,
        m3,
        h13,
        t_report,
    )
}

pub fn mock_rk4fixed_sys() -> System {
    let m1 = ThermalMass::new(1.0, 0.0);
    let m2 = ThermalMass::new(2.0, 10.0);
    let h12 = Conductance::new(5.0);
    let m3 = ThermalMass::new(1.5, 12.0);
    let h13 = Conductance::new(5.0);
    let t_report: Vec<f64> = Vec::linspace(0.0, 1.0, 51);

    System::new(Default::default(), m1, m2, h12, m3, h13, t_report)
}
