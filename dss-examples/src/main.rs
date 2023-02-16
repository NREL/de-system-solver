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
    // build and run prescribed-step Euler system
    let mut sys_euler = mock_euler_sys();

    sys_euler.walk();

    let target_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .unwrap()
        .to_path_buf();
    let dt = sys_euler.t_report[1] - sys_euler.t_report.first().unwrap();

    let mut json_file = target_dir.clone();
    json_file.push(format!("target/results dt={dt} s.json"));

    sys_euler
        .to_file(json_file.as_os_str().to_str().unwrap())
        .unwrap();

    let mut yaml_file = target_dir.clone();
    yaml_file.push(format!("target/results dt={dt} s.yaml"));

    sys_euler
        .to_file(yaml_file.as_os_str().to_str().unwrap())
        .unwrap();

    // build and run prescribed-step 4th-order Runge-Kutta system
    let mut sys_rk4 = mock_rk4fixed_sys();

    sys_rk4.walk();

    let target_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .unwrap()
        .to_path_buf();
    let dt = sys_rk4.t_report[1] - sys_rk4.t_report.first().unwrap();

    let mut json_file = target_dir.clone();
    json_file.push(format!("target/rk4 results dt={dt} s.json"));

    sys_rk4
        .to_file(json_file.as_os_str().to_str().unwrap())
        .unwrap();

    let mut yaml_file = target_dir.clone();
    yaml_file.push(format!("target/rk4 results dt={dt} s.yaml"));

    sys_rk4
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
    let t_report: Vec<f64> = Vec::linspace(0.0, 2.0, 51);

    System::new(SolverOptions::RK4Fixed, m1, m2, h12, m3, h13, t_report)
}