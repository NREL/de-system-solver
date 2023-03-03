use dess_examples::*;

fn main() {
    // build and run prescribed-step Euler system
    let mut sys_euler = mock_euler_sys();

    let t_euler = time_it!(sys_euler.walk());

    let dt = sys_euler.t_report[1] - sys_euler.t_report.first().unwrap();

    println!(
        "Euler {} s time step elapsed time: {} μs",
        dt,
        t_euler.as_micros()
    );

    let overwrite_euler_benchmark: bool = false;
    if overwrite_euler_benchmark {
        let benchmark_file = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .parent()
            .unwrap()
            .to_path_buf()
            .join("dess-examples/tests/fixtures/euler benchmark.yaml");

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

    let overwrite_rk4_benchmark: bool = false;
    if overwrite_rk4_benchmark {
        let benchmark_file = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .parent()
            .unwrap()
            .to_path_buf()
            .join("dess-examples/tests/fixtures/rk4 benchmark.yaml");

        sys_rk4
            .to_file(benchmark_file.as_os_str().to_str().unwrap())
            .unwrap();
    }

    // build and run adaptive RK45
    let mut sys_rk45 = mock_rk45_sys();

    let t_rk45 = time_it!(sys_rk45.walk());

    let dt = sys_rk45.t_report[1] - sys_rk45.t_report.first().unwrap();

    println!(
        "RK45 Adaptive {} s time step elapsed time: {} μs",
        dt,
        t_rk45.as_micros()
    );

    let overwrite_rk45_benchmark: bool = false;
    if overwrite_rk45_benchmark {
        let benchmark_file = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .parent()
            .unwrap()
            .to_path_buf()
            .join("dess-examples/tests/fixtures/rk45 benchmark.yaml");

        sys_rk45
            .to_file(benchmark_file.as_os_str().to_str().unwrap())
            .unwrap();
    }
}
