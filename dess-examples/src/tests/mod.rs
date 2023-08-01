pub mod euler_three_thermal_mass_sys;
pub mod euler_three_thrml_mass_w_bc_sys;
pub mod tests_core;
pub mod dess_core {
    pub mod solver {}
}

#[cfg(test)]
mod method_tests {
    use super::euler_three_thermal_mass_sys;
    use super::euler_three_thrml_mass_w_bc_sys;
    use dess_core::solver::*;
    #[test]
    fn test_eulers_accuracy() {
        println!("Euler's Method:");
        euler_three_thermal_mass_sys::test_method_against_euler_baseline(
            SolverTypes::EulerFixed { dt: 1e-3 },
            5e-3,
        );
        println!("Heun's Method bc:");
        euler_three_thrml_mass_w_bc_sys::test_method_against_euler_baseline_bc(
            SolverTypes::EulerFixed { dt: 1e-3 },
            1e-2,
        );
    }
    #[test]
    fn test_heuns_accuracy() {
        println!("Heun's Method:");
        euler_three_thermal_mass_sys::test_method_against_euler_baseline(
            SolverTypes::HeunsMethod { dt: 1e-3 },
            7.5e-6,
        );
        println!("Heun's Method bc:");
        euler_three_thrml_mass_w_bc_sys::test_method_against_euler_baseline_bc(
            SolverTypes::HeunsMethod { dt: 1e-3 },
            7.5e-3,
        );
    }
    #[test]
    fn test_midpoint_accuracy() {
        println!("Midpoint Method:");
        euler_three_thermal_mass_sys::test_method_against_euler_baseline(
            SolverTypes::MidpointMethod { dt: 1e-3 },
            7.5e-6,
        );
        println!("Midpoint Method bc:");
        euler_three_thrml_mass_w_bc_sys::test_method_against_euler_baseline_bc(
            SolverTypes::MidpointMethod { dt: 1e-3 },
            5e-3,
        );
    }
    #[test]
    fn test_ralstons_accuracy() {
        println!("Ralston's Method:");
        euler_three_thermal_mass_sys::test_method_against_euler_baseline(
            SolverTypes::RalstonsMethod { dt: 1e-3 },
            7.5e-6,
        );
        println!("Ralston's Method bc:");
        euler_three_thrml_mass_w_bc_sys::test_method_against_euler_baseline_bc(
            SolverTypes::RalstonsMethod { dt: 1e-3 },
            7.5e-3,
        );
    }
    #[test]
    fn test_rk4_accuracy() {
        println!("RK4 Method:");
        euler_three_thermal_mass_sys::test_method_against_euler_baseline(
            SolverTypes::RK4Fixed { dt: 1e-3 },
            7.5e-9,
        );
        println!("RK4 Method bc:");
        euler_three_thrml_mass_w_bc_sys::test_method_against_euler_baseline_bc(
            SolverTypes::RK4Fixed { dt: 1e-3 },
            5e-3,
        );
    }
    #[test]
    fn test_rk45_accuracy() {
        println!("RK45 (Cash-Karp) Method:");
        euler_three_thermal_mass_sys::test_method_against_euler_baseline(
            SolverTypes::RK45CashKarp(Box::default()),
            5e-5,
        );
        println!("RK45 (Cash-Karp) Method bc:");
        euler_three_thrml_mass_w_bc_sys::test_method_against_euler_baseline_bc(
            SolverTypes::RK45CashKarp(Box::default()),
            1e-4,
        );
    }
}
