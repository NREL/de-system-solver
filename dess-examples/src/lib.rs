pub mod imports;
pub use imports::*;
pub mod components;
pub use components::*;
mod tests;

/// System of connected components
#[derive(HistoryMethods, BareClone)]
#[pyo3_api]
#[solver]
#[common_derives]
pub struct System {
    #[skip_get]
    solver_opts: SolverOptions,
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

    // boiler plate fields (could be generated with proc macro)
    pub state: SystemState,
    pub history: SystemStateHistoryVec,
}

impl System {
    pub fn new(
        solver_opts: SolverOptions,
        m1: ThermalMass,
        m2: ThermalMass,
        h12: Conductance,
        m3: ThermalMass,
        h13: Conductance,
        t_report: Vec<f64>,
    ) -> Self {
        Self {
            solver_opts,
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

    /// Updates time derivatives of states.
    /// This method must be user defined.
    pub fn update_derivs(&mut self) {
        self.reset_derivs();
        connect_states!(self, (m1, m2, h12), (m1, m3, h13));
        update_derivs!(self, (m1, m2, h12), (m1, m3, h13));
    }
}

#[derive(Copy, HistoryVec)]
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
    let m1 = ThermalMass::new(1.0, 0.0, None);
    let m2 = ThermalMass::new(2.0, 10.0, None);
    let h12 = Conductance::new(5.0, None);
    let m3 = ThermalMass::new(1.5, 12.0, None);
    let h13 = Conductance::new(5.0, None);
    let t_report: Vec<f64> = Vec::linspace(0.0, 1.0, 201);

    System::new(Default::default(), m1, m2, h12, m3, h13, t_report)
}

pub fn mock_rk4fixed_sys() -> System {
    let m1 = ThermalMass::new(1.0, 0.0, None);
    let m2 = ThermalMass::new(2.0, 10.0, None);
    let h12 = Conductance::new(5.0, None);
    let m3 = ThermalMass::new(1.5, 12.0, None);
    let h13 = Conductance::new(5.0, None);
    let t_report: Vec<f64> = Vec::linspace(0.0, 1.0, 51);

    System::new(SolverOptions::RK4Fixed, m1, m2, h12, m3, h13, t_report)
}
