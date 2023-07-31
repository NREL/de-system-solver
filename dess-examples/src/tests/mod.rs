pub mod euler_three_thermal_mass_sys;
pub mod euler_three_thrml_mass_w_bc_sys;
pub mod tests_core;
pub mod dess_core {
    pub mod solver {}
}

#[cfg(test)]
mod tests {
    use super::euler_three_thermal_mass_sys;
    use super::euler_three_thrml_mass_w_bc_sys;
    use dess_core::solver::*;
    #[test]
    fn test_method_accuracy() {
        println!("Heun's Method:");
        euler_three_thermal_mass_sys::test_method_against_euler_baseline(
            SolverTypes::HeunsMethod { dt: 1e-3 },
            1e-2,
        );
        println!("Midpoint Method:");
        euler_three_thermal_mass_sys::test_method_against_euler_baseline(
            SolverTypes::MidpointMethod { dt: 1e-3 },
            1e-2,
        );
        println!("Ralston's Method:");
        euler_three_thermal_mass_sys::test_method_against_euler_baseline(
            SolverTypes::RalstonsMethod { dt: 1e-3 },
            1e-2,
        );
        println!("RK4 Method:");
        euler_three_thermal_mass_sys::test_method_against_euler_baseline(
            SolverTypes::RK4Fixed { dt: 1e-3 },
            1e-2,
        );
        println!("RK45 (Cash-Karp) Method:");
        euler_three_thermal_mass_sys::test_method_against_euler_baseline(
            SolverTypes::RK45CashKarp(Box::default()),
            1e-2,
        );
        println!("Heun's Method:");
        euler_three_thrml_mass_w_bc_sys::test_method_against_euler_baseline_bc(
            SolverTypes::HeunsMethod { dt: 1e-3 },
            1e-2,
        );
        println!("Midpoint Method:");
        euler_three_thrml_mass_w_bc_sys::test_method_against_euler_baseline_bc(
            SolverTypes::MidpointMethod { dt: 1e-3 },
            1e-2,
        );
        println!("Ralston's Method:");
        euler_three_thrml_mass_w_bc_sys::test_method_against_euler_baseline_bc(
            SolverTypes::RalstonsMethod { dt: 1e-3 },
            1e-2,
        );
        println!("RK4 Method:");
        euler_three_thrml_mass_w_bc_sys::test_method_against_euler_baseline_bc(
            SolverTypes::RK4Fixed { dt: 1e-3 },
            1e-2,
        );
        println!("RK45 (Cash-Karp) Method:");
        euler_three_thrml_mass_w_bc_sys::test_method_against_euler_baseline_bc(
            SolverTypes::RK45CashKarp(Box::default()),
            1e-2,
        );
    }
}
