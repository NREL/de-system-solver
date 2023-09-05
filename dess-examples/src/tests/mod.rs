pub mod euler_three_thermal_mass_sys;
pub mod euler_three_thrml_mass_w_bc_sys;
pub mod tests_core;
pub mod dess_core {
    pub mod solver {}
}
pub struct AdaptiveSolverConfig;

#[cfg(test)]
mod method_tests {
    use super::euler_three_thermal_mass_sys::*;
    use super::euler_three_thrml_mass_w_bc_sys::*;
    use dess_core::solver::*;
    #[test]
    fn test_eulers_accuracy() {
        println!("Euler's Method:");
        test_method_against_euler_baseline(SolverTypes::EulerFixed { dt: 1e-3 }, 5e-3);
        println!("Euler's Method bc:");
        test_method_against_euler_baseline_bc(
            SolverTypes::EulerFixed {
                dt: 0.013888888888888886,
            },
            1e-3,
        );
    }
    #[test]
    fn test_heuns_accuracy() {
        println!("Heun's Method:");
        test_method_against_euler_baseline(
            SolverTypes::HeunsMethod {
                dt: 0.1,
            },
            5e-3,
        );
        println!("Heun's Method bc:");
        test_method_against_euler_baseline_bc(
            SolverTypes::HeunsMethod {
                dt: 0.1,
            },
            1e-2,
        );
    }
    #[test]
    fn test_midpoint_accuracy() {
        println!("Midpoint Method:");
        test_method_against_euler_baseline(
            SolverTypes::MidpointMethod {
                dt: 0.1,
            },
            5e-3,
        );
        println!("Midpoint Method bc:");
        test_method_against_euler_baseline_bc(
            SolverTypes::MidpointMethod {
                dt: 0.013888888888888892,
            },
            1e-2,
        );
    }
    #[test]
    fn test_ralstons_accuracy() {
        println!("Ralston's Method:");
        test_method_against_euler_baseline(
            SolverTypes::RalstonsMethod {
                dt: 0.09090909090909091,
            },
            2.5e-2,
        );
        println!("Ralston's Method bc:");
        test_method_against_euler_baseline_bc(
            SolverTypes::RalstonsMethod {
                dt: 0.014285714285714282,
            },
            1e-4,
        );
    }
    #[test]
    fn test_rk23_accuracy() {
        println!("RK23 (Bogacki-Shampine) Method:");
        test_method_against_euler_baseline(
            SolverTypes::RK23BogackiShampine(Box::new(AdaptiveSolverConfig {
                rtol: 1e-2,
                atol: 1e-3,
                state: SolverState {
                    dt: 0.1,
                    ..Default::default()
                },
                ..Default::default()
            })),
            1e-2,
        );
        println!("RK23 (Bogacki-Shampine) Method bc:");
        test_method_against_euler_baseline_bc(
            SolverTypes::RK23BogackiShampine(Box::new(AdaptiveSolverConfig {
                rtol: 1e-2,
                atol: 1e-3,
                ..Default::default()
            })),
            5e-4,
        );
    }
    #[test]
    fn test_rk4_accuracy() {
        println!("RK4 Method:");
        test_method_against_euler_baseline(
            SolverTypes::RK4Fixed {
                dt: 0.05555555555555555,
            },
            7.5e-5,
        );
        println!("RK4 Method bc:");
        test_method_against_euler_baseline_bc(
            SolverTypes::RK4Fixed {
                dt: 0.038461538461538464,
            },
            7.5e-3,
        );
    }
    #[test]
    fn test_rk45_accuracy() {
        println!("RK45 (Cash-Karp) Method:");
        test_method_against_euler_baseline(SolverTypes::RK45CashKarp(Box::default()), 2.5e-6);
        println!("RK45 (Cash-Karp) Method bc:");
        test_method_against_euler_baseline_bc(
            SolverTypes::RK45CashKarp(Box::new(AdaptiveSolverConfig {
                rtol: 1e-3,
                atol: 1e-5,
                ..Default::default()
            })),
            2.5e-3,
        );
    }
}
