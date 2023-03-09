use crate::components::*;
use crate::imports::*;
use plotters::prelude::*;

/// System of connected components
#[derive(HistoryMethods, BareClone)]
#[pyo3_api(
    #[new]
    fn __new__(
        solver_type: String,
        m1: ThermalReservoir,
        m2: ThermalMass,
        h12: Conductance,
        m3: ThermalMass,
        h23: Conductance,
        t_report: Vec<f64>,
    ) -> Self {
        Self::new(
            SolverTypes::from_json(&solver_type).unwrap(),
            m1,
            m2,
            h12,
            m3,
            h23,
            t_report,
        )
    }

    #[classmethod]
    fn new_rk45_cash_karp(
        _cls: &PyType,
        sol: AdaptiveSolverConfig,
        m1: ThermalReservoir,
        m2: ThermalMass,
        h12: Conductance,
        m3: ThermalMass,
        h23: Conductance,
        t_report: Vec<f64>,
    ) -> Self {
        Self::new(
            SolverTypes::RK45CashKarp(sol),
            m1,
            m2,
            h12,
            m3,
            h23,
            t_report
        )
    }

    #[getter]
    fn get_solver_conf(&self) -> Option<AdaptiveSolverConfig> {
        match &self.solver_type {
            SolverTypes::RK45CashKarp(sc) => Some(sc.clone()),
            _ => None,
        }
    }

    #[getter]
    fn get_solver_type(&self) -> String {
        self.solver_type.to_json()
    }

    #[pyo3(name = "walk")]
    fn walk_py(&mut self) {
        self.walk();
    }
)]
#[solver(
    /// Updates time derivatives of states.
    /// This method must be user defined in `solver` macro args.
    fn update_derivs(&mut self) {
        self.reset_derivs();
        connect_states!(self, (m1, m2, h12), (m2, m3, h23));
        update_derivs!(self, (m1, m2, h12), (m2, m3, h23));
        if self.state.time > 0.5 {
            // change in temperature to test what the solver does
            self.m1.state.temp = 10.0;
        }
    }
)]
#[common_derives]
#[derive(Default)]
pub struct System3TMWithBC {
    #[skip_get]
    solver_type: SolverTypes,
    // components
    #[use_state]
    pub m1: ThermalReservoir,
    #[use_state]
    pub m2: ThermalMass,
    /// h12 connects m1 to m2
    #[save_state]
    pub h12: Conductance,
    #[use_state]
    pub m3: ThermalMass,
    #[save_state]
    pub h23: Conductance,
    // fields needed by `solver` procedural macro
    pub t_report: Vec<f64>,
    pub state: SystemState,
    pub history: SystemStateHistoryVec,
}

impl System3TMWithBC {
    pub fn new(
        solver_type: SolverTypes,
        m1: ThermalReservoir,
        m2: ThermalMass,
        h12: Conductance,
        m3: ThermalMass,
        h23: Conductance,
        t_report: Vec<f64>,
    ) -> Self {
        Self {
            solver_type,
            m1,
            m2,
            h12,
            m3,
            h23,
            t_report,
            state: Default::default(),
            history: Default::default(),
        }
    }
}

#[derive(Copy, HistoryVec, Default)]
#[common_derives]
#[pyo3_api]
pub struct SystemState3TM {
    // current index in `t_report`
    i: usize,
    // current time
    time: f64,
}

pub fn mock_euler_sys() -> System3TMWithBC {
    let m1 = ThermalReservoir::new(-1.0);
    let m2 = ThermalMass::new(2.0, 10.0);
    let h12 = Conductance::new(5.0);
    let m3 = ThermalMass::new(1.5, 12.0);
    let h23 = Conductance::new(5.0);
    let t_report: Vec<f64> = Vec::linspace(0.0, 1.0, 201);

    System3TMWithBC::new(
        SolverTypes::EulerFixed { dt: 5e-3 },
        m1,
        m2,
        h12,
        m3,
        h23,
        t_report,
    )
}

pub fn mock_rk4fixed_sys() -> System3TMWithBC {
    let t_report: Vec<f64> = Vec::linspace(0.0, 1.0, 51);

    System3TMWithBC {
        solver_type: Default::default(),
        t_report,
        ..mock_euler_sys()
    }
}

pub fn mock_rk45_sys() -> System3TMWithBC {
    let t_report: Vec<f64> = Vec::linspace(0.0, 1.0, 11);

    System3TMWithBC {
        solver_type: SolverTypes::RK45CashKarp(AdaptiveSolverConfig::default()),
        t_report,
        ..mock_euler_sys()
    }
}

pub fn run_three_tm_w_bc_sys() {
    let overwrite_benchmarks = false;
    // build and run prescribed-step Euler system
    let mut sys_euler = mock_euler_sys();

    let t_euler = time_it!(sys_euler.walk());

    let dt = sys_euler.t_report[1] - sys_euler.t_report.first().unwrap();

    println!(
        "Euler {} s time step elapsed time: {} μs",
        dt,
        t_euler.as_micros()
    );

    let overwrite_euler_benchmark: bool = overwrite_benchmarks;
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

    let overwrite_rk4_benchmark: bool = overwrite_benchmarks;
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
        "RK45 Adaptive {} s init time step elapsed time: {} μs",
        dt,
        t_rk45.as_micros()
    );

    let overwrite_rk45_benchmark: bool = overwrite_benchmarks;
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

    let mut sys_rk45_with_save = System3TMWithBC {
        solver_type: SolverTypes::RK45CashKarp(AdaptiveSolverConfig {
            save: true,
            max_iter: 5,
            ..Default::default()
        }),
        t_report: vec![0.0, 0.33, 0.66, 1.0],
        ..mock_rk45_sys()
    };
    sys_rk45_with_save.walk();
    plot_stuff(sys_rk45_with_save);
}

pub fn plot_stuff(sys: System3TMWithBC) {
    let out_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .unwrap()
        .to_path_buf()
        .join("target/artifacts/");
    DirBuilder::new()
        .recursive(true)
        .create(out_dir.clone())
        .unwrap();
    let out_file = out_dir.join("sys_3TM_w_bc_rk45.png");
    let out_file_str = out_file.as_os_str().to_str().unwrap();

    let root_area = BitMapBackend::new(&out_file, (1024, 768)).into_drawing_area();

    root_area.fill(&WHITE).unwrap();

    let root_height = root_area.dim_in_pixel().1;

    let (upper, lower) = root_area.split_vertically(root_height * 2 / 3);
    let (upper, middle) = upper.split_vertically(root_height / 3);

    plot_upper(sys.clone(), &upper);
    plot_middle(sys.clone(), &middle);
    plot_bottom(sys, &lower);

    // To avoid the IO failure being ignored silently, we manually call the present function
    root_area.present().unwrap();
    println!("Result has been saved to {out_file_str}");
}

fn plot_upper(
    sys: System3TMWithBC,
    root_area: &DrawingArea<BitMapBackend, plotters::coord::Shift>,
) {
    let _title_font_size = 40;

    let ymin = (sys
        .m1
        .history
        .temp
        .iter()
        .fold(f64::INFINITY, |prev, curr| prev.min(*curr))
        .min(
            sys.m2
                .history
                .temp
                .iter()
                .fold(f64::INFINITY, |prev, curr| prev.min(*curr)),
        )
        .min(
            sys.m3
                .history
                .temp
                .iter()
                .fold(f64::INFINITY, |prev, curr| prev.min(*curr)),
        )) as f32;
    let ymax = (sys
        .m1
        .history
        .temp
        .iter()
        .fold(f64::NEG_INFINITY, |prev, curr| prev.max(*curr))
        .max(
            sys.m2
                .history
                .temp
                .iter()
                .fold(f64::NEG_INFINITY, |prev, curr| prev.max(*curr)),
        )
        .max(
            sys.m3
                .history
                .temp
                .iter()
                .fold(f64::NEG_INFINITY, |prev, curr| prev.max(*curr)),
        )) as f32;
    let yrange = ymax - ymin;
    let ymax = ymax + 0.05 * yrange;
    let ymin = ymin - 0.05 * yrange;

    let mut cc = ChartBuilder::on(root_area)
        .x_label_area_size(10)
        .y_label_area_size(100)
        .margin(25)
        // .caption(format!("y = x^{}", 1 + 2), ("sans-serif", title_font_size))
        .build_cartesian_2d(0.0..sys.state.time as f32, ymin..ymax)
        .unwrap();

    let label_font_size = 30;
    cc.configure_mesh()
        .x_labels(5)
        .y_labels(3)
        .y_desc("Temp.")
        .y_label_style(("sans-serif", label_font_size))
        .x_label_style(("sans-serif", label_font_size))
        // .x_desc("Time [s]")
        .max_light_lines(4)
        .draw()
        .unwrap();

    cc.draw_series(LineSeries::new(
        sys.history
            .time
            .iter()
            .zip(sys.m1.history.temp.clone())
            .map(|(x, y)| (*x as f32, y as f32)),
        &RED,
    ))
    .unwrap()
    .label("m1")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    cc.draw_series(LineSeries::new(
        sys.history
            .time
            .iter()
            .zip(sys.m2.history.temp.clone())
            .map(|(x, y)| (*x as f32, y as f32)),
        &GREEN,
    ))
    .unwrap()
    .label("m2")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    cc.draw_series(LineSeries::new(
        sys.history
            .time
            .iter()
            .zip(sys.m3.history.temp.clone())
            .map(|(x, y)| (*x as f32, y as f32)),
        &BLUE,
    ))
    .unwrap()
    .label("m1")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    cc.configure_series_labels()
        .border_style(&BLACK)
        .label_font(("sans-serif", 30))
        .position(SeriesLabelPosition::LowerRight)
        .draw()
        .unwrap();

    cc.draw_series(PointSeries::of_element(
        sys.history
            .time
            .iter()
            .zip(sys.m1.history.temp)
            .map(|(x, y)| (*x as f32, y as f32)),
        5,
        ShapeStyle::from(&RED).filled(),
        &|coord, size, style| {
            EmptyElement::at(coord) + Circle::new((0, 0), size, style)
            // + Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
        },
    ))
    .unwrap();

    cc.draw_series(PointSeries::of_element(
        sys.history
            .time
            .iter()
            .zip(sys.m2.history.temp)
            .map(|(x, y)| (*x as f32, y as f32)),
        5,
        ShapeStyle::from(&GREEN).filled(),
        &|coord, size, style| {
            EmptyElement::at(coord) + Circle::new((0, 0), size, style)
            // + Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
        },
    ))
    .unwrap();

    cc.draw_series(PointSeries::of_element(
        sys.history
            .time
            .iter()
            .zip(sys.m3.history.temp)
            .map(|(x, y)| (*x as f32, y as f32)),
        5,
        ShapeStyle::from(&BLUE).filled(),
        &|coord, size, style| {
            EmptyElement::at(coord) + Circle::new((0, 0), size, style)
            // + Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
        },
    ))
    .unwrap();
}

fn plot_middle(
    sys: System3TMWithBC,
    root_area: &DrawingArea<BitMapBackend, plotters::coord::Shift>,
) {
    let ymin = 0.0;
    let sc = match sys.solver_type {
        SolverTypes::RK45CashKarp(sc) => sc,
        _ => unimplemented!(),
    };
    let ymax = sc
        .history
        .norm_err
        .iter()
        .zip(sc.history.norm_err_rel.clone())
        .fold(f64::NEG_INFINITY, |prev, (curr1, curr2)| {
            curr1
                .unwrap_or_default()
                .max(curr2.unwrap_or_default().max(prev))
        }) as f32;
    let yrange = ymax - ymin;
    let ymax = ymax + 0.05 * yrange;
    let ymin = ymin - 0.05 * yrange;

    let mut cc = ChartBuilder::on(root_area)
        .x_label_area_size(70)
        .y_label_area_size(100)
        .margin(25)
        // .caption(format!("y = x^{}", 1 + 2), ("sans-serif", title_font_size))
        .build_cartesian_2d(0.0..sys.state.time as f32, ymin..ymax)
        .unwrap();

    let label_font_size = 30;
    cc.configure_mesh()
        .x_labels(5)
        .y_labels(3)
        .y_desc("Error Norm")
        .y_label_style(("sans-serif", label_font_size))
        .x_label_style(("sans-serif", label_font_size))
        // .x_desc("Time [s]")
        .max_light_lines(4)
        .draw()
        .unwrap();

    cc.draw_series(LineSeries::new(
        sc.history
            .t_curr
            .iter()
            .zip(
                sc.history
                    .norm_err_rel
                    .iter()
                    .map(|x| x.unwrap_or_default()),
            )
            .map(|(x, y)| (*x as f32, y as f32)),
        &RED,
    ))
    .unwrap()
    .label("relative")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    cc.draw_series(LineSeries::new(
        sc.history
            .t_curr
            .iter()
            .zip(sc.history.norm_err.iter().map(|x| x.unwrap_or_default()))
            .map(|(x, y)| (*x as f32, y as f32)),
        &GREEN,
    ))
    .unwrap()
    .label("absolute")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    cc.configure_series_labels()
        .border_style(&BLACK)
        .label_font(("sans-serif", 30))
        .position(SeriesLabelPosition::UpperRight)
        .draw()
        .unwrap();

    cc.draw_series(PointSeries::of_element(
        sc.history
            .t_curr
            .iter()
            .zip(
                sc.history
                    .norm_err_rel
                    .iter()
                    .map(|x| x.unwrap_or_default()),
            )
            .map(|(x, y)| (*x as f32, y as f32)),
        5,
        ShapeStyle::from(&RED).filled(),
        &|coord, size, style| {
            EmptyElement::at(coord) + Circle::new((0, 0), size, style)
            // + Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
        },
    ))
    .unwrap();

    cc.draw_series(PointSeries::of_element(
        sc.history
            .t_curr
            .iter()
            .zip(sc.history.norm_err.iter().map(|x| x.unwrap_or_default()))
            .map(|(x, y)| (*x as f32, y as f32)),
        5,
        ShapeStyle::from(&GREEN).filled(),
        &|coord, size, style| {
            EmptyElement::at(coord) + Circle::new((0, 0), size, style)
            // + Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
        },
    ))
    .unwrap();
}

fn plot_bottom(
    sys: System3TMWithBC,
    root_area: &DrawingArea<BitMapBackend, plotters::coord::Shift>,
) {
    let ymin = 0.0;
    let sc = match sys.solver_type {
        SolverTypes::RK45CashKarp(sc) => sc,
        _ => unimplemented!(),
    };
    let ymax = (sc
        .history
        .dt
        .iter()
        .fold(f64::NEG_INFINITY, |prev, curr| curr.max(prev))) as f32;
    let yrange = ymax - ymin;
    let ymax = ymax + 0.05 * yrange;
    let ymin = ymin - 0.05 * yrange;

    let mut cc = ChartBuilder::on(root_area)
        .x_label_area_size(70)
        .y_label_area_size(100)
        .margin(25)
        // .caption(format!("y = x^{}", 1 + 2), ("sans-serif", title_font_size))
        .build_cartesian_2d(0.0..sys.state.time as f32, ymin..ymax)
        .unwrap();

    let label_font_size = 30;
    cc.configure_mesh()
        .x_labels(5)
        .y_labels(3)
        .y_desc("dt")
        .y_label_style(("sans-serif", label_font_size))
        .x_label_style(("sans-serif", label_font_size))
        .x_desc("Time [s]")
        .max_light_lines(4)
        .draw()
        .unwrap();

    cc.draw_series(LineSeries::new(
        sc.history
            .t_curr
            .iter()
            .zip(sc.history.dt.clone())
            .map(|(x, y)| (*x as f32, y as f32)),
        &RED,
    ))
    .unwrap();

    cc.draw_series(PointSeries::of_element(
        sc.history
            .t_curr
            .iter()
            .zip(sc.history.dt)
            .map(|(x, y)| (*x as f32, y as f32)),
        5,
        ShapeStyle::from(&RED).filled(),
        &|coord, size, style| {
            EmptyElement::at(coord) + Circle::new((0, 0), size, style)
            // + Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
        },
    ))
    .unwrap();
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_bare_clone() {
//         let mut sys = mock_euler_sys();
//         assert!(sys.history.is_empty());
//         assert!(sys.m1.history.is_empty());
//         assert!(sys.h12.history.is_empty());
//         sys.save_state();
//         // verify that at least a couple of the expected changes happened
//         assert!(sys.history.len() == 1);
//         assert!(sys.m1.history.len() == 1);
//         assert!(sys.h12.history.len() == 1);
//         let bare_sys = sys.bare_clone();
//         assert!(bare_sys.history.is_empty());
//         assert!(bare_sys.m1.history.is_empty());
//         assert!(bare_sys.h12.history.is_empty());
//     }

//     #[test]
//     fn test_euler_against_benchmark() {
//         let mut sys = mock_euler_sys();
//         sys.walk();

//         let benchmark_file = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
//             .parent()
//             .unwrap()
//             .to_path_buf()
//             .join("dess-examples/tests/fixtures/euler benchmark.yaml");

//         let benchmark_sys =
//             System3TM::from_file(benchmark_file.as_os_str().to_str().unwrap()).unwrap();
//         assert_eq!(sys, benchmark_sys);
//     }

//     #[test]
//     fn test_rk4_against_benchmark() {
//         let mut sys = mock_rk4fixed_sys();
//         sys.walk();
//         let benchmark_file = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
//             .parent()
//             .unwrap()
//             .to_path_buf()
//             .join("dess-examples/tests/fixtures/rk4 benchmark.yaml");

//         let benchmark_sys =
//             System3TM::from_file(benchmark_file.as_os_str().to_str().unwrap()).unwrap();
//         assert_eq!(sys, benchmark_sys);
//     }

//     #[test]
//     fn test_rk4_dt_behavior() {
//         let base_sys = mock_rk4fixed_sys();

//         // system for checking if small dt results in relatively higher accuracy
//         let mut sys_dt_smaller_than_t_report = System3TM {
//             solver_type: SolverTypes::RK4Fixed { dt: 1e-3 },
//             ..base_sys.clone()
//         };
//         sys_dt_smaller_than_t_report.walk();

//         // system for checking if dt slightly less than t_report works ok
//         let mut sys_dt_slightly_less_than_t_report = System3TM {
//             solver_type: SolverTypes::RK4Fixed {
//                 dt: (base_sys.t_report[1].clone() - base_sys.t_report[0].clone()) * 0.9,
//             },
//             ..base_sys.clone()
//         };
//         sys_dt_slightly_less_than_t_report.walk();

//         assert!(
//             sys_dt_smaller_than_t_report.m1.history
//                 != sys_dt_slightly_less_than_t_report.m1.history
//         );

//         // system for checking that t_report overrides dt when dt is slightly larger than t_report
//         let mut sys_dt_slightly_larger_than_t_report = System3TM {
//             solver_type: SolverTypes::RK4Fixed {
//                 dt: (base_sys.t_report[1].clone() - base_sys.t_report[0].clone()) * 1.1,
//             },
//             ..base_sys.clone()
//         };

//         sys_dt_slightly_larger_than_t_report.walk();

//         // system for checking that t_report overrides dt when dt is large
//         let mut sys_dt_larger_than_t_report = System3TM {
//             solver_type: SolverTypes::RK4Fixed {
//                 dt: (base_sys.t_report[1].clone() - base_sys.t_report[0].clone()) * 10.0,
//             },
//             ..base_sys.clone()
//         };
//         sys_dt_larger_than_t_report.walk();

//         assert!(
//             sys_dt_larger_than_t_report.m1.history != sys_dt_slightly_less_than_t_report.m1.history
//         );
//         assert!(
//             sys_dt_larger_than_t_report.m1.history
//                 == sys_dt_slightly_larger_than_t_report.m1.history
//         );
//     }

//     #[test]
//     fn test_rk45_against_benchmark() {
//         let mut sys = mock_rk45_sys();
//         sys.walk();
//         let benchmark_file = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
//             .parent()
//             .unwrap()
//             .to_path_buf()
//             .join("dess-examples/tests/fixtures/rk45 benchmark.yaml");

//         let benchmark_sys =
//             System3TM::from_file(benchmark_file.as_os_str().to_str().unwrap()).unwrap();
//         assert_eq!(sys, benchmark_sys);
//     }
// }
