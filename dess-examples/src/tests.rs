#![cfg(test)]
use super::*;

#[test]
fn test_bare_clone() {
    let mut sys = mock_euler_sys();
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
fn test_euler_against_benchmark() {
    let mut sys = mock_euler_sys();
    sys.walk();

    let benchmark_file = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .unwrap()
        .to_path_buf()
        .join("dess-examples/tests/fixtures/euler benchmark.yaml");

    let benchmark_sys = System::from_file(benchmark_file.as_os_str().to_str().unwrap()).unwrap();
    assert_eq!(sys, benchmark_sys);
}

#[test]
fn test_rk4_against_benchmark() {
    let mut sys = mock_rk4fixed_sys();
    sys.walk();
    let benchmark_file = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .unwrap()
        .to_path_buf()
        .join("dess-examples/tests/fixtures/rk4 benchmark.yaml");

    let benchmark_sys = System::from_file(benchmark_file.as_os_str().to_str().unwrap()).unwrap();
    assert_eq!(sys, benchmark_sys);
}

#[test]
fn test_rk4_dt_behavior() {
    let base_sys = mock_rk4fixed_sys();

    // system for checking if small dt results in relatively higher accuracy
    let mut sys_dt_smaller_than_t_report = System {
        solver_conf: SolverOptions::RK4Fixed { dt: 1e-3 },
        ..base_sys.clone()
    };
    sys_dt_smaller_than_t_report.walk();

    // system for checking if dt slightly less than t_report works ok
    let mut sys_dt_slightly_less_than_t_report = System {
        solver_conf: SolverOptions::RK4Fixed {
            dt: (base_sys.t_report[1].clone() - base_sys.t_report[0].clone()) * 0.9,
        },
        ..base_sys.clone()
    };
    sys_dt_slightly_less_than_t_report.walk();

    assert!(
        sys_dt_smaller_than_t_report.m1.history != sys_dt_slightly_less_than_t_report.m1.history
    );

    // system for checking that t_report overrides dt when dt is slightly larger than t_report
    let mut sys_dt_slightly_larger_than_t_report = System {
        solver_conf: SolverOptions::RK4Fixed {
            dt: (base_sys.t_report[1].clone() - base_sys.t_report[0].clone()) * 1.1,
        },
        ..base_sys.clone()
    };

    sys_dt_slightly_larger_than_t_report.walk();

    // system for checking that t_report overrides dt when dt is large
    let mut sys_dt_larger_than_t_report = System {
        solver_conf: SolverOptions::RK4Fixed {
            dt: (base_sys.t_report[1].clone() - base_sys.t_report[0].clone()) * 10.0,
        },
        ..base_sys.clone()
    };
    sys_dt_larger_than_t_report.walk();

    assert!(
        sys_dt_larger_than_t_report.m1.history != sys_dt_slightly_less_than_t_report.m1.history
    );
    assert!(
        sys_dt_larger_than_t_report.m1.history == sys_dt_slightly_larger_than_t_report.m1.history
    );
}
