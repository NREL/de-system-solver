use dess_examples::*;

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
            .join("dess-examples/tests/fixtures/rk4 benchmark.yaml");

        sys_rk4
            .to_file(benchmark_file.as_os_str().to_str().unwrap())
            .unwrap();
    }
}
