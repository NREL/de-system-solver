use dess_examples::tests::euler_three_thermal_mass_sys::*;
use dess_examples::tests::euler_three_thrml_mass_w_bc_sys::*;
use dess_examples::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // Creating benchmarks for each method so that any changes that result in different values will be flagged
    // Default value is false if no argument is provided
    // To change benchmarks, change value to true
    let mut overwrite_benchmarks = false;

    if args.len() > 1 {
        // Check if the second argument is "true"
        if args[1] == "true" {
            overwrite_benchmarks = true;
        }
    }
    three_thermal_mass_sys::run_three_tm_sys(overwrite_benchmarks);
    three_thrml_mass_w_bc_sys::run_three_tm_w_bc_sys();

    // Creating small step euler baseline to compare to other methods
    let mut overwrite_baseline = false;

    if args.len() > 1 {
        // Check if the second argument is "true"
        if args[1] == "true" {
            overwrite_baseline = true;
        }
    }
    baseline_three_tm_sys(overwrite_baseline);
    baseline_three_tm_w_bc_sys(overwrite_baseline);
}
