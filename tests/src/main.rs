use tests::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Default value is false if no argument is provided -- if true, euler small step baseline will be rewritten
    let mut overwrite_baseline = false;

    if args.len() > 1 {
        // Check if the second argument is "true"
        if args[1] == "true" {
            overwrite_baseline = true;
        }
    }
    euler_three_thermal_mass_sys::baseline_three_tm_sys(overwrite_baseline);
    euler_three_thrml_mass_w_bc_sys::baseline_three_tm_w_bc_sys(overwrite_baseline);
    euler_three_thermal_mass_sys::test_method_against_euler_baseline(
        dess_core::solver::SolverTypes::EulerFixed { dt: 5e-5 },
        5e-10,
    );
    euler_three_thermal_mass_sys::test_method_against_euler_baseline(
        dess_core::solver::SolverTypes::HeunsMethod { dt: 1e-2 },
        1e-1,
    );
    euler_three_thermal_mass_sys::test_method_against_euler_baseline(
        dess_core::solver::SolverTypes::MidpointMethod { dt: 1e-2 },
        1e-1,
    );
    euler_three_thermal_mass_sys::test_method_against_euler_baseline(
        dess_core::solver::SolverTypes::RalstonsMethod { dt: 1e-2 },
        1e-1,
    );
    euler_three_thermal_mass_sys::test_method_against_euler_baseline(
        dess_core::solver::SolverTypes::RK4Fixed { dt: 1e-2 },
        1e-1,
    );
    euler_three_thermal_mass_sys::test_method_against_euler_baseline(
        dess_core::solver::SolverTypes::RK45CashKarp(Box::default()),
        0.025,
    );
    euler_three_thrml_mass_w_bc_sys::test_method_against_euler_baseline(
        dess_core::solver::SolverTypes::EulerFixed { dt: 5e-5 },
        5e-10,
    );
    euler_three_thrml_mass_w_bc_sys::test_method_against_euler_baseline(
        dess_core::solver::SolverTypes::HeunsMethod { dt: 1e-2 },
        0.1,
    );
    euler_three_thrml_mass_w_bc_sys::test_method_against_euler_baseline(
        dess_core::solver::SolverTypes::MidpointMethod { dt: 1e-2 },
        0.05,
    );
    euler_three_thrml_mass_w_bc_sys::test_method_against_euler_baseline(
        dess_core::solver::SolverTypes::RalstonsMethod { dt: 1e-2 },
        0.1,
    );
    euler_three_thrml_mass_w_bc_sys::test_method_against_euler_baseline(
        dess_core::solver::SolverTypes::RK4Fixed { dt: 1e-2 },
        0.05,
    );
    euler_three_thrml_mass_w_bc_sys::test_method_against_euler_baseline(
        dess_core::solver::SolverTypes::RK45CashKarp(Box::default()),
        0.05,
    );
}
