#![cfg(test)]
use crate::*;

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
