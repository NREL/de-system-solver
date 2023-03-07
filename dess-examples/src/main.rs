use dess_examples::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Default value is false if no argument is provided
    let mut overwrite_benchmarks = false;

    if args.len() > 1 {
        // Check if the second argument is "true"
        if args[1] == "true" {
            overwrite_benchmarks = true;
        }
    }
    three_thermal_mass_sys::run_three_tm_sys(overwrite_benchmarks);
    three_thrml_mass_w_bc_sys::run_three_tm_w_bc_sys();
}
