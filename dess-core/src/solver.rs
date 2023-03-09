use crate::imports::*;

#[common_derives]
pub enum SolverTypes {
    /// Euler with fixed time step.
    /// parameter `dt` provides time step size for whenever solver is between
    /// `t_report` times.  
    EulerFixed {
        dt: f64,
    },
    /// Runge-Kutta 4th order with fixed time step.  
    /// parameter `dt` provides time step size for whenever solver is between
    /// `t_report` times.  
    RK4Fixed {
        dt: f64,
    },
    // TODO: add this stuff back into fixed options
    // /// time step to use if `t_report` is larger than `dt`
    // dt: f64,
    /// Runge-Kutta 4/5 order adaptive, Cash-Karp method
    /// https://en.wikipedia.org/wiki/Cash%E2%80%93Karp_method
    RK45CashKarp(AdaptiveSolverConfig),
    ToDo,
}

impl Default for SolverTypes {
    fn default() -> Self {
        SolverTypes::RK4Fixed { dt: 0.1 }
    }
}

#[pyo3_api(
    #[new]
    fn new_py(
        dt_init: f64,
        dt_max: Option<f64>,
        max_iter: Option<u8>,
        rtol: Option<f64>,
        atol: Option<f64>,
        save: Option<bool>,
    ) -> Self {
        Self::new(
            dt_init,
            dt_max,
            max_iter,
            rtol,
            atol,
            save.unwrap_or_default(),
        )
    }
)]
#[common_derives]
pub struct AdaptiveSolverConfig {
    /// max allowable dt
    pub dt_max: f64,
    /// max number of iterations per time step
    pub max_iter: u8,
    /// absolute euclidean error tolerance
    pub atol: f64,
    /// relative euclidean error tolerance
    pub rtol: f64,
    /// save iteration history
    pub save: bool,
    /// solver state
    pub state: SolverState,
    /// history of solver state
    pub history: SolverStateHistoryVec,
}

impl AdaptiveSolverConfig {
    pub fn new(
        dt_init: f64,
        dt_max: Option<f64>,
        max_iter: Option<u8>,
        rtol: Option<f64>,
        atol: Option<f64>,
        save: bool,
    ) -> Self {
        let mut state = SolverState::default();
        state.dt = dt_init;
        Self {
            dt_max: dt_max.unwrap_or(10.0),
            max_iter: max_iter.unwrap_or(2),
            rtol: rtol.unwrap_or(1e-6),
            atol: atol.unwrap_or(1e-12),
            save,
            state,
            history: Default::default(),
        }
    }
}

impl Default for AdaptiveSolverConfig {
    fn default() -> Self {
        Self::new(0.1, None, None, None, None, false)
    }
}

impl AsMut<AdaptiveSolverConfig> for AdaptiveSolverConfig {
    fn as_mut(&mut self) -> &mut AdaptiveSolverConfig {
        self
    }
}

#[common_derives]
#[pyo3_api]
#[derive(HistoryVec)]
/// Solver is considered considered converged when any one of the following conditions are met:
/// - `norm_err` is less than `atol`
/// - `norm_err_rel` is less than `rtol`
/// - `n_iter` >= `n_max_iter`
pub struct SolverState {
    /// time step size used by solver
    pub dt: f64,
    /// number of iterations to achieve tolerance
    pub n_iter: u8,
    /// Absolute error based on difference in L2 (euclidean) norm
    pub norm_err: Option<f64>,
    /// Relative error based on difference in L2 (euclidean) norm
    pub norm_err_rel: Option<f64>,
    /// current system time used in solver
    pub t_curr: f64,
    /// current values of states
    pub states: Vec<f64>,
}

impl Default for SolverState {
    fn default() -> Self {
        Self {
            dt: 0.1,
            n_iter: 0,
            norm_err: None,
            norm_err_rel: None,
            t_curr: 0.0,
            states: Default::default(),
        }
    }
}

pub trait SolverBase: BareClone + Sized {
    /// assuming `set_derivs` has been called, steps
    /// value of states by deriv * dt
    fn step_by_dt(&mut self, dt: &f64);

    /// steps dt without affecting states
    fn step_time(&mut self, dt: &f64);

    /// assuming `set_derivs` has been called, steps
    /// value of states by deriv * dt
    fn step(&mut self, val: Vec<f64>);

    /// reset all time derivatives to zero for start of `solve_step`
    fn reset_derivs(&mut self);

    /// returns derivatives of states
    fn get_derivs(&self) -> Vec<f64>;

    /// sets values of derivatives of states
    fn set_derivs(&mut self, val: &Vec<f64>);
    /// returns values of states
    fn get_states(&self) -> Vec<f64>;

    /// sets values of states
    fn set_states(&mut self, val: Vec<f64>);

    /// Updates time derivatives of states.
    /// This method must be user defined.
    fn update_derivs(&mut self);

    /// Returns `solver_conf`, if applicable
    fn sc(&self) -> Option<&AdaptiveSolverConfig>;

    /// Returns mut `solver_conf`, if applicable
    fn sc_mut(&mut self) -> Option<&mut AdaptiveSolverConfig>;

    /// Returns [Self::state]
    fn state(&self) -> &crate::SystemState;
}

pub trait SolverVariantMethods: SolverBase {
    /// Steps forward by `dt`
    fn euler(&mut self, dt: &f64) {
        self.update_derivs();
        self.step_by_dt(dt);
    }

    /// solves time step with 4th order Runge-Kutta method.
    /// See RK4 method: https://en.wikipedia.org/wiki/Runge%E2%80%93Kutta_methods#Examples
    fn rk4fixed(&mut self, dt: &f64) {
        self.update_derivs();

        // k1 = f(x_i, y_i)
        let k1s = self.get_derivs();

        // k2 = f(x_i + 1 / 2 * h, y_i + 1 / 2 * k1 * h)
        let mut sys1 = self.bare_clone();
        sys1.step_by_dt(&(dt / 2.0));
        sys1.update_derivs();
        let k2s = sys1.get_derivs();

        // k3 = f(x_i + 1 / 2 * h, y_i + 1 / 2 * k2 * h)
        let mut sys2 = self.bare_clone();
        sys2.set_derivs(&k2s);
        sys2.step_by_dt(&(dt / 2.0));
        sys2.update_derivs();
        let k3s = sys2.get_derivs();

        // k4 = f(x_i + h, y_i + k3 * h)
        let mut sys3 = self.bare_clone();
        sys3.set_derivs(&k3s);
        sys3.step_by_dt(&dt);
        sys3.update_derivs();
        let k4s = sys3.get_derivs();

        let mut delta: Vec<f64> = vec![];
        let zipped = zip!(k1s, k2s, k3s, k4s);
        for (k1, (k2, (k3, k4))) in zipped {
            delta.push(1.0 / 6.0 * (k1 + 2.0 * k2 + 2.0 * k3 + k4) * dt);
        }

        self.step(delta);
        self.step_time(dt);
    }

    /// solves time step with adaptive Cash-Karp Method (variant of RK45) and returns `dt` used
    /// https://en.wikipedia.org/wiki/Cash%E2%80%93Karp_method
    fn rk45_cash_karp(&mut self, dt_max: &f64) -> f64 {
        let sc_mut = self.sc_mut().unwrap();
        // reset iteration counter
        sc_mut.state.n_iter = 0;
        sc_mut.state.dt = sc_mut.state.dt.min(dt_max.clone());

        // loop to find `dt` that results in meeting tolerance
        // and does not exceed `dt_max`
        let delta5 = loop {
            let sc = self.sc().unwrap();
            let dt = sc.state.dt;

            // run a single step at `dt`
            let (delta4, delta5) = self.rk45_cash_karp_step(dt);

            // reborrow because of the borrow above in `self.rk45_cash_karp_step(dt);`
            let sc = self.sc().unwrap();
            // grab states for later use if solver steps are to be saved
            let states = if sc.save {
                self.get_states()
                    .clone()
                    .iter()
                    .zip(delta5.clone())
                    .map(|(s, d)| s + d)
                    .collect::<Vec<f64>>()
            } else {
                vec![]
            };

            let t_curr = self.state().time.clone();

            // mutably borrow sc to update it
            let sc = self.sc_mut().unwrap();

            // update `n_iter`, `norm_err`, `norm_err_rel`, `t_curr`, and `states`
            // still need to update dt at some point
            sc.state.n_iter += 1;
            sc.state.norm_err = Some(
                delta4
                    .iter()
                    .zip(&delta5)
                    .map(|(d4, d5)| (d4 - d5).powi(2))
                    .collect::<Vec<f64>>()
                    .iter()
                    .sum::<f64>()
                    .sqrt(),
            );
            let norm_d5 = delta5
                .iter()
                .map(|d5| d5.powi(2))
                .collect::<Vec<f64>>()
                .iter()
                .sum::<f64>()
                .sqrt();

            sc.state.norm_err_rel = if norm_d5 > sc.atol {
                // `unwrap` is ok here because `norm_err` will always be some by this point
                Some(sc.state.norm_err.unwrap() / norm_d5)
            } else {
                // avoid dividing by a really small denominator
                None
            };
            // pretty sure `dt` needs to be added here, as is being done
            sc.state.t_curr = t_curr + dt;

            sc.state.states = states;

            // conditions for breaking loop
            // if there is a relative error, use that
            // otherwise, use the absolute error
            let tol_met = match sc.state.norm_err_rel {
                Some(norm_err_rel) => norm_err_rel <= sc.rtol,
                None => match sc.state.norm_err {
                    Some(norm_err) => norm_err <= sc.atol,
                    None => unreachable!(),
                },
            };

            // Because we need to be able to possibly expand the next time step,
            // regardless of whether break condition is met,
            // adapt dt based on `rtol` if it is Some; use `atol` otherwise
            // this adaptation strategy came directly from Chapra and Canale's section on adapting the time step
            let dt_coeff = match sc.state.norm_err_rel {
                Some(norm_err_rel) => {
                    (sc.rtol / norm_err_rel).powf(if norm_err_rel <= sc.rtol { 0.2 } else { 0.25 })
                }
                None => {
                    match sc.state.norm_err {
                        Some(norm_err) => {
                            (sc.atol / norm_err).powf(if norm_err <= sc.atol { 0.2 } else { 0.25 })
                        }
                        None => 1.0, // don't adapt if there is not enough information to do so
                    }
                }
            };
            sc.state.dt *= dt_coeff;

            // if tolerance is achieved here, then we proceed to the next time step, and
            // `dt` will be limited to `dt_max` at the start of the next time step.  If tolerance
            // is not achieved, then time step will be decreased.
            let break_cond =
                sc.state.n_iter >= sc.max_iter || sc.state.norm_err.unwrap() < sc.atol || tol_met;

            if break_cond {
                if sc.save {
                    sc.history.push(sc.state.clone());
                }
                break delta5;
            };
        };

        // increment forward with 5th order solution
        self.step(delta5);
        let sc = self.sc().unwrap();
        let dt_used = sc.state.dt;
        self.step_time(&dt_used);
        // dbg!(self.state.time);
        // dbg!(self.t_report[self.state.i]);
        dt_used
    }

    fn rk45_cash_karp_step(&mut self, dt: f64) -> (Vec<f64>, Vec<f64>) {
        self.update_derivs();

        // k1 = f(x_i, y_i)
        let k1s = self.get_derivs();

        // k2 = f(x_i + 1 / 5 * h, y_i + 1 / 5 * k1 * h)
        let mut sys1 = self.bare_clone();
        sys1.step_by_dt(&(dt / 5.0));
        sys1.update_derivs();
        let k2s = sys1.get_derivs();

        // k3 = f(x_i + 3 / 10 * h, y_i + 3 / 40 * k1 * h + 9 / 40 * k2 * h)
        let mut sys2 = self.bare_clone();
        sys2.set_derivs(&k2s);
        sys2.step_by_dt(&(dt * 3.0 / 10.0));
        sys2.update_derivs();
        let k3s = sys2.get_derivs();

        // k4 = f(x_i + 3 / 5 * h, y_i + 3 / 10 * k1 * h - 9 / 10 * k2 * h + 6 / 5 * k3 * h)
        let mut sys3 = self.bare_clone();
        sys3.set_derivs(&k3s);
        sys3.step_by_dt(&(dt * 3.0 / 5.0));
        sys3.update_derivs();
        let k4s = sys3.get_derivs();

        // k5 = f(x_i + h, y_i - 11 / 54 * k1 * h + 5 / 2 * k2 * h - 70 / 27 * k3 * h + 35 / 27 * k4 * h)
        let mut sys4 = self.bare_clone();
        sys4.set_derivs(&k4s);
        sys4.step_by_dt(&dt);
        sys4.update_derivs();
        let k5s = sys4.get_derivs();

        // k6 = f(x_i + 7 / 8 * h, y_i + 1631 / 55296 * k1 * h + 175 / 512 * k2 * h + 575 / 13824 * k3 * h + 44275 / 110592 * k4 * h + 253 / 4096 * k4 * h)
        let mut sys5 = self.bare_clone();
        sys5.set_derivs(&k5s);
        sys5.step_by_dt(&(dt * 7.0 / 8.0));
        sys5.update_derivs();
        let k6s = sys5.get_derivs();

        // 4th order delta
        let mut delta4: Vec<f64> = vec![];
        // 5th order delta
        let mut delta5: Vec<f64> = vec![];
        let zipped = zip!(k1s, k2s, k3s, k4s, k5s, k6s);
        for (k1, (_k2, (k3, (k4, (k5, k6))))) in zipped {
            delta4.push(
                (37. / 378. * k1 + 250. / 621. * k3 + 125. / 594. * k4 + 512. / 1_771. * k6) * dt,
            );
            delta5.push(
                (2825. / 27_648. * k1
                    + 18_575. / 48_384. * k3
                    + 13_525. / 55_296. * k4
                    + 277. / 14_336. * k5
                    + 1.0 / 4.0 * k6)
                    * dt,
            );
        }
        (delta4, delta5)
    }
}
