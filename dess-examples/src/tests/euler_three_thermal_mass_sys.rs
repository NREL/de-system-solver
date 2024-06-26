use crate::imports::*;
use crate::three_thermal_mass_sys::System3TM;
use dess::solver::SolverTypes;
use eng_fmt::FormatEng;
/// building and running small step (high accuracy) euler method for system3TM comparison
pub fn baseline_euler_sys() -> System3TM {
    let t_report: Vec<f64> = Vec::linspace(0.0, 1.0, 4);

    System3TM {
        solver_type: SolverTypes::EulerFixed { dt: 5e-10 },
        t_report,
        ..Default::default()
    }
}
/// creating a baseline .yaml file with small step euler for comparison, when overwrite_baseline=true in main.rs
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
            .join("dess-examples/src/tests/fixtures/euler_baseline.yaml");

        sys_euler
            .to_file(baseline_file.as_os_str().to_str().unwrap())
            .unwrap();
    }
}
/// three thermal mass with chosen method, to compare to small step euler
pub fn mock_method_sys(method: SolverTypes) -> System3TM {
    let t_report: Vec<f64> = Vec::linspace(0.0, 1.0, 4);

    System3TM {
        solver_type: method,
        t_report,
        ..Default::default()
    }
}
/// tests chosen solver (including dt) against small step euler
pub fn test_method_against_euler_baseline(method: SolverTypes, epsilon: f64) {
    let mut sys = mock_method_sys(method);
    sys.walk();
    // taking baseline
    let baseline_file = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .unwrap()
        .to_path_buf()
        .join("dess-examples/src/tests/fixtures/euler_baseline.yaml");
    let baseline_sys = System3TM::from_file(baseline_file.as_os_str().to_str().unwrap()).unwrap();
    // temperatures for m1, m2, m3 with small step euler
    let baseline_m1 = baseline_sys.m1.history.temp;
    let baseline_m2 = baseline_sys.m2.history.temp;
    let baseline_m3 = baseline_sys.m3.history.temp;
    // temperatures for m1, m2, m3 with chosen method
    let method_m1 = sys.m1.history.temp;
    let method_m2 = sys.m2.history.temp;
    let method_m3 = sys.m3.history.temp;
    let m1: Vec<(&f64, &f64)> = baseline_m1.iter().zip(&method_m1).collect();
    let m2: Vec<(&f64, &f64)> = baseline_m2.iter().zip(&method_m2).collect();
    let m3: Vec<(&f64, &f64)> = baseline_m3.iter().zip(&method_m3).collect();
    let m1_within_epsilon = crate::tests::tests_core::within_epsilon(m1, epsilon);
    println!(
        "Stays within {} of m1 solution: {}",
        epsilon.format_eng(Some(2)),
        m1_within_epsilon
    );
    let m2_within_epsilon = crate::tests::tests_core::within_epsilon(m2, epsilon);
    println!(
        "Stays within {} of m2 solution: {}",
        epsilon.format_eng(Some(2)),
        m2_within_epsilon
    );
    let m3_within_epsilon = crate::tests::tests_core::within_epsilon(m3, epsilon);
    println!(
        "Stays within {} of m3 solution: {}",
        epsilon.format_eng(Some(2)),
        m3_within_epsilon
    );
    let m1_1: Vec<(&f64, &f64)> = baseline_m1.iter().zip(&method_m1).collect();
    let m2_1: Vec<(&f64, &f64)> = baseline_m2.iter().zip(&method_m2).collect();
    let m3_1: Vec<(&f64, &f64)> = baseline_m3.iter().zip(&method_m3).collect();
    let m1_mean_absolute_error = crate::tests::tests_core::average_distance(m1_1);
    println!(
        "Mean absolute error of m1 solution: {}",
        m1_mean_absolute_error.format_eng(Some(10))
    );
    let m2_mean_absolute_error = crate::tests::tests_core::average_distance(m2_1);
    println!(
        "Mean absolute error of m2 solution: {}",
        m2_mean_absolute_error.format_eng(Some(10))
    );
    let m3_mean_absolute_error = crate::tests::tests_core::average_distance(m3_1);
    println!(
        "Mean absolute error of m3 solution: {}",
        m3_mean_absolute_error.format_eng(Some(10))
    )
}
