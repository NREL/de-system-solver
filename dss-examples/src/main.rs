use std::path::PathBuf;

use dss_core::prelude::*;

pub mod imports;
use imports::*;
pub mod components;
pub use components::*;

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

    /// Steps forward by `dt` and returns Vec of state derivatives
    pub fn step(&mut self, dt: &f64) {
        self.reset_derivs();
        connect_states!(self, (m1, m2, h12, m1, m3, h13), dt);
        update_derivs!(self, (m1, m2, h12, m1, m3, h13), dt);
        self.step_states(dt);
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
    let mut system = mock_system();

    system.walk();

    let mut temp_file = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .unwrap()
        .to_path_buf();
    let dt = system.t_report[1] - system.t_report.first().unwrap();
    temp_file.push(format!("target/results dt={dt} s.json"));

    system
        .to_file(temp_file.as_os_str().to_str().unwrap())
        .unwrap();

    // TODO: make a test around this
    // dbg!(system.bare_clone());
}

pub fn mock_system() -> System {
    let m1 = ThermalMass::new(1.0, 0.0, None);
    let m2 = ThermalMass::new(2.0, 10.0, None);
    let h12 = Conductance::new(5.0, None);
    let m3 = ThermalMass::new(1.5, 12.0, None);
    let h13 = Conductance::new(5.0, None);
    let t_report: Vec<f64> = Vec::linspace(0.0, 2.0, 201);
    dbg!(&t_report);

    System::new(Default::default(), m1, m2, h12, m3, h13, t_report)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bare_clone() {
        let mut sys = mock_system();
        assert!(sys.history.is_empty());
        assert!(sys.m1.history.is_empty());
        assert!(sys.h12.history.is_empty());
        sys.save_state();
        // verify that at least a couple of the expected changes happened
        assert!(sys.history.len() == 1);
        assert!(sys.m1.history.len() == 1);
        assert!(sys.h12.history.len() == 1);
        let bare_sys = sys.bare_clone();
        assert!(bare_sys.history.is_empty());
        assert!(bare_sys.m1.history.is_empty());
        assert!(bare_sys.h12.history.is_empty());
    }

    #[test]
    fn test_against_benchmark() {
        let mut sys = mock_system();
        sys.walk();

        let mut temp_file = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .parent()
            .unwrap()
            .to_path_buf();
        temp_file.push("dss-examples/tests/fixtures/benchmark.yaml");

        let benchmark_sys = System::from_file(temp_file.as_os_str().to_str().unwrap()).unwrap();
        assert_eq!(sys, benchmark_sys);
    }
}
