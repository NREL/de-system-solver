use std::fs::DirBuilder;
use std::path::PathBuf;
use std::time::Instant;

use dss_core::prelude::*;

pub mod imports;
use imports::*;
pub mod components;
pub use components::*;
mod tests;

/// System of connected components
#[derive(HistoryMethods, BareClone, Pyo3Api)]
#[solver]
#[common_derives]
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

#[derive(Copy, HistoryVec)]
#[common_derives]
pub struct SystemState {
    // current index in `t_report`
    i: usize,
    // current time
    time: f64,
}

macro_rules! time_it {
    ($thing: expr) => {{
        let t0 = Instant::now();
        $thing;
        let t_elapsed = Instant::now() - t0;
        t_elapsed
    }};
}

fn main() {
    // build and run prescribed-step Euler system
    let mut sys_euler = mock_euler_sys();

    let t_euler = time_it!(sys_euler.walk());

    let results_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .unwrap()
        .to_path_buf()
        .join("target/results/");
    let mut dir_bulder = DirBuilder::new();
    dir_bulder
        .recursive(true)
        .create(results_dir.clone())
        .unwrap();

    let dt = sys_euler.t_report[1] - sys_euler.t_report.first().unwrap();

    println!(
        "Euler {} s time step elapsed time: {} μs",
        dt,
        t_euler.as_micros()
    );

    let mut json_file = results_dir.clone();
    json_file.push(format!("euler dt={dt} s.json"));

    sys_euler
        .to_file(json_file.as_os_str().to_str().unwrap())
        .unwrap();

    let mut yaml_file = results_dir.clone();
    yaml_file.push(format!("euler dt={dt} s.yaml"));

    sys_euler
        .to_file(yaml_file.as_os_str().to_str().unwrap())
        .unwrap();

    let overwrite_euler_benchmark: bool = false;
    if overwrite_euler_benchmark {
        let benchmark_file = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .parent()
            .unwrap()
            .to_path_buf()
            .join("dss-examples/tests/fixtures/euler benchmark.yaml");

        sys_euler
            .to_file(benchmark_file.as_os_str().to_str().unwrap())
            .unwrap();
    }

    // build and run prescribed-step 4th-order Runge-Kutta system
    let mut sys_rk4 = mock_rk4fixed_sys();

    let t_rk4 = time_it!(sys_rk4.walk());

    let dt = sys_rk4.t_report[1] - sys_rk4.t_report.first().unwrap();

    println!(
        "RK4 {} s time step elapsed time: {} μs",
        dt,
        t_rk4.as_micros()
    );

    let mut json_file = results_dir.clone();
    json_file.push(format!("rk4 dt={dt} s.json"));

    sys_rk4
        .to_file(json_file.as_os_str().to_str().unwrap())
        .unwrap();

    let mut yaml_file = results_dir.clone();
    yaml_file.push(format!("rk4 dt={dt} s.yaml"));

    sys_rk4
        .to_file(yaml_file.as_os_str().to_str().unwrap())
        .unwrap();

    let overwrite_rk_benchmark: bool = false;
    if overwrite_rk_benchmark {
        let benchmark_file = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .parent()
            .unwrap()
            .to_path_buf()
            .join("dss-examples/tests/fixtures/rk4 benchmark.yaml");

        sys_rk4
            .to_file(benchmark_file.as_os_str().to_str().unwrap())
            .unwrap();
    }
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
