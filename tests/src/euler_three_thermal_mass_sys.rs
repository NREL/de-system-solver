use crate::tests_core::*;
use dess_core::prelude::*;
use dess_core::solver::SolverTypes;
use dess_examples::components::*;
use dess_examples::imports::*;
use dess_examples::three_thermal_mass_sys::System3TM;
///building and running small step (high accuracy) euler method for system3TM comparison
pub fn baseline_euler_sys() -> System3TM {
    let m1 = ThermalMass::new(1.0, 0.0);
    let m2 = ThermalMass::new(2.0, 10.0);
    let h12 = Conductance::new(5.0);
    let m3 = ThermalMass::new(1.5, 12.0);
    let h23 = Conductance::new(5.0);
    let t_report: Vec<f64> = Vec::linspace(0.0, 1.0, 21);

    System3TM::new(
        SolverTypes::EulerFixed { dt: 5e-10 },
        m1,
        m2,
        h12,
        m3,
        h23,
        t_report,
    )
}
///creating a baseline .yaml file with small step euler for comparison, when overwrite_baseline=true in main.rs
pub fn baseline_three_tm_sys(overwrite_baseline: bool) {
    if overwrite_baseline {
        let mut sys_euler = baseline_euler_sys();

        let t_euler = time_it!(sys_euler.walk());

        let dt = sys_euler.t_report[1] - sys_euler.t_report.first().unwrap();

        println!(
            "Euler {} s time step elapsed time: {} μs",
            dt,
            t_euler.as_micros()
        );
        let baseline_file = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .parent()
            .unwrap()
            .to_path_buf()
            .join("tests/fixtures/euler_baseline.yaml");

        sys_euler
            .to_file(baseline_file.as_os_str().to_str().unwrap())
            .unwrap();
    }
}
///three thermal mass with chosen method, to compare to small step euler
pub fn mock_method_sys(method: SolverTypes) -> System3TM {
    let m1 = ThermalMass::new(1.0, 0.0);
    let m2 = ThermalMass::new(2.0, 10.0);
    let h12 = Conductance::new(5.0);
    let m3 = ThermalMass::new(1.5, 12.0);
    let h23 = Conductance::new(5.0);
    let t_report: Vec<f64> = Vec::linspace(0.0, 1.0, 21);

    System3TM::new(method, m1, m2, h12, m3, h23, t_report)
}
///tests chosen solver (including dt) against small step euler
pub fn test_method_against_euler_baseline(method: SolverTypes, epsilon: f64) {
    let mut sys = mock_method_sys(method);
    sys.walk();
    //taking baseline
    let baseline_file = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .unwrap()
        .to_path_buf()
        .join("tests/fixtures/euler_baseline.yaml");
    let baseline_sys = System3TM::from_file(baseline_file.as_os_str().to_str().unwrap()).unwrap();
    //temperatures for m1, m2, m3 with small step euler
    let baseline_m1 = baseline_sys.m1.history.temp;
    let baseline_m2 = baseline_sys.m2.history.temp;
    let baseline_m3 = baseline_sys.m3.history.temp;
    //temperatures for m1, m2, m3 with chosen method
    let method_m1 = sys.m1.history.temp;
    let method_m2 = sys.m2.history.temp;
    let method_m3 = sys.m3.history.temp;
    //need to create different comparisons -- almost eq, small dt to be within epsilon (might need own function),
    //average difference (abs, rel), greatest distance(abs, rel)
    let m1: Vec<(&f64, &f64)> = baseline_m1.iter().zip(&method_m1).collect();
    let m2: Vec<(&f64, &f64)> = baseline_m2.iter().zip(&method_m2).collect();
    let m3: Vec<(&f64, &f64)> = baseline_m3.iter().zip(&method_m3).collect();
    let m1_new = m1.clone();
    let m2_new = m2.clone();
    let m3_new = m3.clone();
    let m1_new_1 = m1.clone();
    let m2_new_1 = m2.clone();
    let m3_new_1 = m3.clone();
    let m1_within_epsilon = within_epsilon(m1, epsilon);
    println!(
        "Stays within {} of m1 solution: {}",
        epsilon, m1_within_epsilon
    );
    let m2_within_epsilon = within_epsilon(m2, epsilon);
    println!(
        "Stays within {} of m2 solution: {}",
        epsilon, m2_within_epsilon
    );
    let m3_within_epsilon = within_epsilon(m3, epsilon);
    println!(
        "Stays within {} of m3 solution: {}",
        epsilon, m3_within_epsilon
    );
    let m1_within_epsilon_absolute_error_only = within_epsilon_absolute_error_only(m1_new, epsilon);
    println!(
        "Stays within {} of m1 solution: {}",
        epsilon, m1_within_epsilon_absolute_error_only
    );
    let m2_within_epsilon_absolute_error_only = within_epsilon_absolute_error_only(m2_new, epsilon);
    println!(
        "Stays within {} of m2 solution: {}",
        epsilon, m2_within_epsilon_absolute_error_only
    );
    let m3_within_epsilon_absolute_error_only = within_epsilon_absolute_error_only(m3_new, epsilon);
    println!(
        "Stays within {} of m3 solution: {}",
        epsilon, m3_within_epsilon_absolute_error_only
    );
    let average_distance_m1 = average_distance(m1_new_1);
    println!(
        "The average distance between method and baseline for three thermal mass m1 solution is {}.",
        average_distance_m1
    );
    let average_distance_m2 = average_distance(m2_new_1);
    println!(
        "The average distance between method and baseline for three thermal mass m2 solution is {}.",
        average_distance_m2
    );
    let average_distance_m3 = average_distance(m3_new_1);
    println!(
        "The average distance between method and baseline for three thermal mass m3 solution is {}.",
        average_distance_m3
    );
}
