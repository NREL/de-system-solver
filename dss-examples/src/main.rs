use std::path::PathBuf;

use dss_core::prelude::*;

pub mod imports;
use imports::*;
pub mod components;
pub use components::*;
mod tests;

/// System of connected components
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
    HistoryMethods,
    SystemSolver,
    BareClone,
)]
pub struct System {
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

#[derive(
    Debug, Default, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, HistoryVec,
)]
pub struct SystemState {
    // current index in `t_report`
    i: usize,
    // current time
    time: f64,
}

fn main() {
    let mut system = mock_euler_sys();

    system.walk();

    let target_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .unwrap()
        .to_path_buf();
    let dt = system.t_report[1] - system.t_report.first().unwrap();

    let mut json_file = target_dir.clone();
    json_file.push(format!("target/results dt={dt} s.json"));

    system
        .to_file(json_file.as_os_str().to_str().unwrap())
        .unwrap();

    let mut yaml_file = target_dir.clone();
    yaml_file.push(format!("target/results dt={dt} s.yaml"));

    system
        .to_file(yaml_file.as_os_str().to_str().unwrap())
        .unwrap();
}

pub fn mock_euler_sys() -> System {
    let m1 = ThermalMass::new(1.0, 0.0, None);
    let m2 = ThermalMass::new(2.0, 10.0, None);
    let h12 = Conductance::new(5.0, None);
    let m3 = ThermalMass::new(1.5, 12.0, None);
    let h13 = Conductance::new(5.0, None);
    let t_report: Vec<f64> = Vec::linspace(0.0, 2.0, 201);

    System::new(Default::default(), m1, m2, h12, m3, h13, t_report)
}

pub fn mock_rk4fixed_sys() -> System {
    let m1 = ThermalMass::new(1.0, 0.0, None);
    let m2 = ThermalMass::new(2.0, 10.0, None);
    let h12 = Conductance::new(5.0, None);
    let m3 = ThermalMass::new(1.5, 12.0, None);
    let h13 = Conductance::new(5.0, None);
    let t_report: Vec<f64> = Vec::linspace(0.0, 2.0, 201);

    System::new(SolverOptions::RK4Fixed, m1, m2, h12, m3, h13, t_report)
}
