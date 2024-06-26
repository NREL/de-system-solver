use crate::imports::*;

#[common_derives]
pub enum SolverTypes {
    /// Euler with fixed time step.
    /// parameter `dt` provides time step size for whenever solver is between
    /// `t_report` times.  ≥
    EulerFixed { dt: f64 },
    /// Heun's Method. (basic Runge-Kutta 2nd order with fixed time step)
    HeunsMethod { dt: f64 },
    /// Midpoint Method. ( alternate Runge-Kutta 2nd order with fixed time step)
    MidpointMethod { dt: f64 },
    /// Ralston's Method. ( alternate Runge-Kutta 2nd order with fixed time step)
    RalstonsMethod { dt: f64 },
    /// Bogacki-Shampine Method. Runge-Kutte 2/3 order adaptive solver
    RK23BogackiShampine(Box<AdaptiveSolverConfig>),
    /// Runge-Kutta 4th order with fixed time step
    /// parameter `dt` provides time step size for whenever solver is between
    /// `t_report` times.  
    RK4Fixed { dt: f64 },
    // TODO: add this stuff back into fixed options
    // /// time step to use if `t_report` is larger than `dt`
    // dt: f64,
    /// Runge-Kutta 4/5 order adaptive, Cash-Karp method
    /// https://en.wikipedia.org/wiki/Cash%E2%80%93Karp_method
    RK45CashKarp(Box<AdaptiveSolverConfig>),
    // TODO: add more variants here
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
        dt_max: f64,
        max_iter: u8,
        rtol: f64,
        atol: f64,
        save: Option<bool>,
        save_states: Option<bool>,
    ) -> Self {
        Self{
            dt_max,
            max_iter,
            atol,
            rtol,
            save: save.unwrap_or(false),
            save_states: save_states.unwrap_or(false),
            state: SolverState {
                dt: dt_init,
                ..Default::default()
            },
            history: Default::default(),
        }
    }

    #[pyo3(name = "dt_mean")]
    fn dt_mean_py(&self) -> Option<f64> {
        self.dt_mean()
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
    /// save states in iteration history
    /// this is computationally expensive and should be generally `false`
    pub save_states: bool,
    /// solver state
    pub state: SolverState,
    /// history of solver state
    pub history: SolverStateHistoryVec,
}

impl Default for AdaptiveSolverConfig {
    fn default() -> Self {
        Self {
            dt_max: 10.,
            max_iter: 5,
            rtol: 1e-5,
            atol: 1e-9,
            save: false,
            save_states: false,
            state: SolverState {
                dt: 0.1,
                ..Default::default()
            },
            history: Default::default(),
        }
    }
}

impl AdaptiveSolverConfig {
    pub fn dt_mean(&self) -> Option<f64> {
        if !self.history.is_empty() {
            Some(self.history.dt.iter().fold(0., |acc, &x| acc + x) / self.history.len() as f64)
        } else {
            None
        }
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
            t_curr: 0.,
            states: Default::default(),
        }
    }
}

pub trait SolverBase: HasStates + Sized {
    /// reset all time derivatives to zero for start of `solve_step`
    fn reset_derivs(&mut self);
    /// Updates time derivatives of states.
    /// This method must be user defined.
    fn update_derivs(&mut self);
    /// steps dt without affecting states
    fn step_time(&mut self, dt: &f64);
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
        self.step_states_by_dt(dt);
        self.update_derivs();
    }
    /// Heun's Method (starts out with Euler's method but adds an extra step)
    /// See Heun's Method (the first listed Heun's method, not the one also known as Ralston's Method):
    /// https://en.wikipedia.org/wiki/Heun%27s_method
    fn heun(&mut self, dt: &f64) {
        self.update_derivs();
        //making copy without history, to avoid stepping dt twice
        let mut updated_self = self.bare_clone();
        //recording initial derivative value for later use
        let deriv_0: Vec<f64> = self.derivs();
        //this will give euler's formula result
        self.step_states_by_dt(dt);
        self.update_derivs();
        //recording derivative at endpoint of euler's method line
        let deriv_1: Vec<f64> = self.derivs();
        //creating new vector that is average of deriv_1 and deriv_2
        let deriv_mean: Vec<f64> = deriv_0
            .iter()
            .zip(&deriv_1)
            .map(|(d_1, d_2)| d_1 * 0.5 + d_2 * 0.5)
            .collect::<Vec<f64>>();
        //updates derivative in updated_self to be the average of deriv_0 and deriv_1
        updated_self.set_derivs(&deriv_mean);
        //steps states using the average derivative
        updated_self.step_states_by_dt(dt);
        //saving updated state
        let new_state = updated_self.states();
        //setting state to be the updated state
        self.set_states(new_state);
        self.update_derivs();
    }
    /// Midpoint Method
    /// See: https://en.wikipedia.org/wiki/Midpoint_method
    fn midpoint(&mut self, dt: &f64) {
        self.update_derivs();
        //making copy without history, to avoid stepping dt twice
        let mut updated_self = self.bare_clone();
        //updating time and state to midpoint of line
        updated_self.step_states_by_dt(&(0.5 * dt));
        updated_self.update_derivs();
        //recording derivative at midpoint
        let deriv_1: Vec<f64> = updated_self.derivs();
        //updates derivative in self to be deriv_1
        self.set_derivs(&deriv_1);
        //steps states using the midpoint derivative
        self.step_states_by_dt(dt);
        self.update_derivs();
    }
    /// Ralston's Method
    /// See Ralston's Method: https://en.wikipedia.org/wiki/List_of_Runge%E2%80%93Kutta_methods#Ralston.27s_method
    fn ralston(&mut self, dt: &f64) {
        self.update_derivs();
        //making copy without history, to avoid stepping dt twice
        let mut updated_self = self.bare_clone();
        //recording initial derivative for later
        let deriv_0: Vec<f64> = updated_self.derivs();
        //updating time and state to 2/3 way through line
        updated_self.step_states_by_dt(&(2.0 * dt / 3.0));
        updated_self.update_derivs();
        //recording derivative at 2/3 way through line
        let deriv_1: Vec<f64> = updated_self.derivs();
        //creating new vector that is weighted average of deriv_0 and deriv_1
        let deriv_mean: Vec<f64> = deriv_0
            .iter()
            .zip(&deriv_1)
            .map(|(d_1, d_2)| d_1 / 4.0 + 3.0 * d_2 / 4.0)
            .collect::<Vec<f64>>();
        //updates derivative in self to be deriv_mean
        self.set_derivs(&deriv_mean);
        //steps states using deriv_mean
        self.step_states_by_dt(dt);
        self.update_derivs();
    }
    ///solves time step with adaptive Bogacki Shampine Method (variant of RK23) and returns 'dt' used
    ///see: https://en.wikipedia.org/wiki/Bogacki%E2%80%93Shampine_method
    fn rk23_bogacki_shampine(&mut self, dt_max: &f64) -> f64 {
        let sc_mut = self.sc_mut().unwrap();
        // reset iteration counter
        sc_mut.state.n_iter = 0;
        sc_mut.state.dt = sc_mut.state.dt.min(*dt_max).min(sc_mut.dt_max);

        // loop to find `dt` that results in meeting tolerance
        // and does not exceed `dt_max`
        let (delta3, dt_used) = loop {
            let sc = self.sc().unwrap();
            let dt = sc.state.dt;

            // run a single step at `dt`
            let (delta2, delta3) = self.rk23_bogacki_shampine_step(dt);

            // reborrow because of the borrow above in `self.rk23_bogacki_shampine_step(dt);`
            let sc = self.sc().unwrap();
            // grab states for later use if solver steps are to be saved
            let states = if sc.save {
                self.states()
                    .clone()
                    .iter()
                    .zip(delta3.clone())
                    .map(|(s, d)| s + d)
                    .collect::<Vec<f64>>()
            } else {
                vec![]
            };

            let t_curr = self.state().time;

            // mutably borrow sc to update it
            let sc_mut = self.sc_mut().unwrap();

            // update `n_iter`, `norm_err`, `norm_err_rel`, `t_curr`, and `states`
            // still need to update dt at some point
            sc_mut.state.n_iter += 1;
            // different way of calculating norm -- could add in via an enum later
            // let mut length = 0.;
            // for _item in &delta2 {
            //     length += 1.;
            // }
            // sc_mut.state.norm_err = Some(
            //     delta2
            //         .iter()
            //         .zip(&delta3)
            //         .map(|(d2, d3)| (((d2 - d3).powi(2)).sqrt()))
            //         .collect::<Vec<f64>>()
            //         .iter()
            //         .sum::<f64>()
            //         / length,
            // );
            // let norm_d3 = delta3
            //     .iter()
            //     .map(|d3| (d3.powi(2)).sqrt())
            //     .collect::<Vec<f64>>()
            //     .iter()
            //     .sum::<f64>()
            //     / length;
            sc_mut.state.norm_err = Some(
                delta2
                    .iter()
                    .zip(&delta3)
                    .map(|(d2, d3)| (d2 - d3).powi(2))
                    .collect::<Vec<f64>>()
                    .iter()
                    .sum::<f64>()
                    .sqrt(),
            );
            let norm_d3 = delta3
                .iter()
                .map(|d3| d3.powi(2))
                .collect::<Vec<f64>>()
                .iter()
                .sum::<f64>()
                .sqrt();
            //making sure that rtol is always considered as long as you don't divide by 0
            sc_mut.state.norm_err_rel = if norm_d3 != 0. {
                // `unwrap` is ok here because `norm_err` will always be some by this point
                Some(sc_mut.state.norm_err.unwrap() / norm_d3)
            } else {
                // avoid dividing by 0
                None
            };

            sc_mut.state.t_curr = t_curr;

            if sc_mut.save_states {
                sc_mut.state.states = states;
            }

            // conditions for breaking loop
            // if there is a relative error, use that
            // otherwise, use the absolute error
            let tol_met = match sc_mut.state.norm_err_rel {
                Some(norm_err_rel) => norm_err_rel <= sc_mut.rtol,
                None => match sc_mut.state.norm_err {
                    Some(norm_err) => norm_err <= sc_mut.atol,
                    None => unreachable!(),
                },
            };

            // Because we need to be able to possibly expand the next time step,
            // regardless of whether break condition is met,
            // adapt dt based on `rtol` if it is Some; use `atol` otherwise
            // this adaptation strategy came directly from Chapra and Canale's section on adapting the time step
            // The approach is to adapt more aggressively to meet rtol when decreasing the time step size
            // than when increasing time step size.
            let dt_coeff = match sc_mut.state.norm_err_rel {
                Some(norm_err_rel) => match sc_mut.state.norm_err {
                    //ensures that if either rtol or atol are met, then the step succeeds
                    //prioritizes rtol -- if both are met, then rtol is used
                    //if no atol exists, just considers rtol
                    Some(norm_err) => {
                        if norm_err_rel <= sc_mut.rtol {
                            (sc_mut.rtol / norm_err_rel).powf(0.2)
                        } else if norm_err <= sc_mut.atol {
                            (sc_mut.atol / norm_err).powf(0.2)
                        } else {
                            0.25
                        }
                    }
                    // (sc_mut.rtol / norm_err_rel).powf(
                    //     if norm_err_rel <= sc_mut.rtol || norm_err <= sc_mut.atol {
                    //         0.2
                    //     } else {
                    //         0.25
                    //     },
                    // ),
                    None => (sc_mut.rtol / norm_err_rel).powf(if norm_err_rel <= sc_mut.rtol {
                        0.2
                    } else {
                        0.25
                    }),
                },
                //if no rtol exists, just consideres atol
                None => {
                    match sc_mut.state.norm_err {
                        Some(norm_err) => (sc_mut.atol / norm_err)
                            .powf(if norm_err <= sc_mut.atol { 0.2 } else { 0.25 }),
                        None => 1., // don't adapt if there is not enough information to do so (if neither atol or rtol exist)
                    }
                }
            };
            // if tolerance is achieved here, then we proceed to the next time step, and
            // `dt` will be limited to `dt_max` at the start of the next time step.  If tolerance
            // is not achieved, then time step will be decreased.
            let break_cond = sc_mut.state.n_iter >= sc_mut.max_iter
                || sc_mut.state.norm_err.unwrap() < sc_mut.atol
                || tol_met;

            if break_cond {
                // save before modifying dt
                if sc_mut.save {
                    sc_mut.history.push(sc_mut.state.clone());
                }
                // store used dt before adapting
                let dt_used = sc_mut.state.dt;
                // adapt for next solver time step
                sc_mut.state.dt *= dt_coeff;
                break (delta3, dt_used);
            };
            // adapt for next iteration in current time step
            sc_mut.state.dt *= dt_coeff;
        };

        // increment forward with 3rd order solution
        self.step_states(delta3);
        self.step_time(&dt_used);
        self.update_derivs();
        // dbg!(self.state.time);
        // dbg!(self.t_report[self.state.i]);
        dt_used
    }
    fn rk23_bogacki_shampine_step(&mut self, dt: f64) -> (Vec<f64>, Vec<f64>) {
        self.update_derivs();

        // k1 = f(t_i, x_i)
        let k1s = self.derivs();

        // k2 = f(t_i + 1 / 2 * h, x_i + 1 / 2 * k1 * h)
        let mut sys1 = self.bare_clone();
        sys1.step_states_by_dt(&(dt / 2.));
        sys1.update_derivs();
        let k2s = sys1.derivs();
        // k3 = f(t_i + 3 / 4 * h, x_i + 3 / 4 * k2 * h)
        let mut sys2 = self.bare_clone();
        sys2.set_derivs(&k2s);
        sys2.step_states_by_dt(&(dt * 3. / 4.));
        sys2.update_derivs();
        let k3s = sys2.derivs();
        // k4 = f(x_i + h, y_i + 2 / 9 * k1 * h + 1 / 3 * k2 * h + 4 / 9 * k3 * h) = 3rd order solution
        let mut sys3 = self.bare_clone();
        sys3.step_time(&(dt));
        // 3nd order delta
        let delta3: Vec<f64> = {
            let (k1s, k2s, k3s) = (k1s.clone(), k2s.clone(), k3s.clone());
            let zipped = zip!(k1s, k2s, k3s);
            let mut steps = vec![];
            for (k1, (k2, k3)) in zipped {
                steps.push((2. / 9. * k1 + 1. / 3. * k2 + 4. / 9. * k3) * dt);
            }
            steps
        };
        let delta3_new = delta3.clone();
        sys3.step_states(delta3_new);
        sys3.update_derivs();
        let k4s = sys3.derivs();
        // 2nd order delta
        let mut delta2: Vec<f64> = vec![];
        let zipped = zip!(k1s, k2s, k3s, k4s);
        for (k1, (k2, (k3, k4))) in zipped {
            delta2.push((7. / 24. * k1 + 1. / 4. * k2 + 1. / 3. * k3 + 1. / 8. * k4) * dt);
        }
        (delta2, delta3)
    }
    /// solves time step with 4th order Runge-Kutta method.
    /// See RK4 method: https://en.wikipedia.org/wiki/Runge%E2%80%93Kutta_methods#Examples
    fn rk4fixed(&mut self, dt: &f64) {
        self.update_derivs();

        // k1 = f(x_i, y_i)
        let k1s = self.derivs();

        // k2 = f(x_i + 1 / 2 * h, y_i + 1 / 2 * k1 * h)
        let mut sys1 = self.bare_clone();
        sys1.step_states_by_dt(&(dt / 2.));
        sys1.update_derivs();
        let k2s = sys1.derivs();

        // k3 = f(x_i + 1 / 2 * h, y_i + 1 / 2 * k2 * h)
        let mut sys2 = self.bare_clone();
        sys2.set_derivs(&k2s);
        sys2.step_states_by_dt(&(dt / 2.));
        sys2.update_derivs();
        let k3s = sys2.derivs();

        // k4 = f(x_i + h, y_i + k3 * h)
        let mut sys3 = self.bare_clone();
        sys3.set_derivs(&k3s);
        sys3.step_states_by_dt(dt);
        sys3.update_derivs();
        let k4s = sys3.derivs();

        let mut delta: Vec<f64> = vec![];
        let zipped = zip!(k1s, k2s, k3s, k4s);
        for (k1, (k2, (k3, k4))) in zipped {
            delta.push(1. / 6. * (k1 + 2. * k2 + 2. * k3 + k4) * dt);
        }

        self.step_states(delta);
        self.step_time(dt);
        self.update_derivs();
    }
    /// solves time step with adaptive Cash-Karp Method (variant of RK45) and returns `dt` used
    /// https://en.wikipedia.org/wiki/Cash%E2%80%93Karp_method
    fn rk45_cash_karp(&mut self, dt_max: &f64) -> f64 {
        let sc_mut = self.sc_mut().unwrap();
        // reset iteration counter
        sc_mut.state.n_iter = 0;
        sc_mut.state.dt = sc_mut.state.dt.min(*dt_max).min(sc_mut.dt_max);

        // loop to find `dt` that results in meeting tolerance
        // and does not exceed `dt_max`
        let (delta5, dt_used) = loop {
            let sc = self.sc().unwrap();
            let dt = sc.state.dt;

            // run a single step at `dt`
            let (delta4, delta5) = self.rk45_cash_karp_step(dt);

            // reborrow because of the borrow above in `self.rk45_cash_karp_step(dt);`
            let sc = self.sc().unwrap();
            // grab states for later use if solver steps are to be saved
            let states = if sc.save {
                self.states()
                    .clone()
                    .iter()
                    .zip(delta5.clone())
                    .map(|(s, d)| s + d)
                    .collect::<Vec<f64>>()
            } else {
                vec![]
            };

            let t_curr = self.state().time;

            // mutably borrow sc to update it
            let sc_mut = self.sc_mut().unwrap();

            // update `n_iter`, `norm_err`, `norm_err_rel`, `t_curr`, and `states`
            // still need to update dt at some point
            sc_mut.state.n_iter += 1;
            //another way to calculate norm -- can be added in later via an enum
            // let mut length = 0.;
            // for _item in &delta4 {
            //     length += 1.;
            // }
            // sc_mut.state.norm_err = Some(
            //     delta4
            //         .iter()
            //         .zip(&delta5)
            //         .map(|(d4, d5)| (((d4 - d5).powi(2)).sqrt()))
            //         .collect::<Vec<f64>>()
            //         .iter()
            //         .sum::<f64>()
            //         / length,
            // );
            // let norm_d5 = delta5
            //     .iter()
            //     .map(|d5| (d5.powi(2)).sqrt())
            //     .collect::<Vec<f64>>()
            //     .iter()
            //     .sum::<f64>()
            //     / length;
            sc_mut.state.norm_err = Some(
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
            //ensures that rtol is calculated and considered as long as you are not dividing by 0
            sc_mut.state.norm_err_rel = if norm_d5 != 0. {
                // `unwrap` is ok here because `norm_err` will always be some by this point
                Some(sc_mut.state.norm_err.unwrap() / norm_d5)
            } else {
                // avoid dividing by 0
                None
            };

            sc_mut.state.t_curr = t_curr;

            if sc_mut.save_states {
                sc_mut.state.states = states;
            }

            // conditions for breaking loop
            // if there is a relative error, use that
            // otherwise, use the absolute error
            let tol_met = match sc_mut.state.norm_err_rel {
                Some(norm_err_rel) => norm_err_rel <= sc_mut.rtol,
                None => match sc_mut.state.norm_err {
                    Some(norm_err) => norm_err <= sc_mut.atol,
                    None => unreachable!(),
                },
            };

            // Because we need to be able to possibly expand the next time step,
            // regardless of whether break condition is met,
            // adapt dt based on `rtol` if it is Some; use `atol` otherwise
            // this adaptation strategy came directly from Chapra and Canale's section on adapting the time step
            // The approach is to adapt more aggressively to meet rtol when decreasing the time step size
            // than when increasing time step size.
            let dt_coeff = match sc_mut.state.norm_err_rel {
                Some(norm_err_rel) => {
                    //ensures that if either rtol or atol are met, then the step succeeds
                    //prioritizes rtol -- if both atol and rtol are met, rtol is used
                    if norm_err_rel <= sc_mut.rtol {
                        (sc_mut.rtol / norm_err_rel).powf(0.2)
                    } else if sc_mut.state.norm_err.unwrap() <= sc_mut.atol {
                        (sc_mut.atol / sc_mut.state.norm_err.unwrap()).powf(0.2)
                    } else {
                        0.25
                    }
                    // (sc_mut.rtol / norm_err_rel).powf(
                    //     if norm_err_rel <= sc_mut.rtol || norm_err <= sc_mut.atol {
                    //         0.2
                    //     } else {
                    //         0.25
                    //     },
                    // ),
                }
                //if rtol doesn't exist just use atol
                None => {
                    match sc_mut.state.norm_err {
                        Some(norm_err) => (sc_mut.atol / norm_err)
                            .powf(if norm_err <= sc_mut.atol { 0.2 } else { 0.25 }),
                        None => 1., // don't adapt if there is not enough information to do so
                    }
                }
            };

            // if tolerance is achieved here, then we proceed to the next time step, and
            // `dt` will be limited to `dt_max` at the start of the next time step.  If tolerance
            // is not achieved, then time step will be decreased.
            let break_cond = sc_mut.state.n_iter >= sc_mut.max_iter
                || sc_mut.state.norm_err.unwrap() < sc_mut.atol
                || tol_met;

            if break_cond {
                // save before modifying dt
                if sc_mut.save {
                    sc_mut.history.push(sc_mut.state.clone());
                }
                // store used dt before adapting
                let dt_used = sc_mut.state.dt;
                // adapt for next solver time step
                sc_mut.state.dt *= dt_coeff;
                break (delta5, dt_used);
            };
            // adapt for next iteration in current time step
            sc_mut.state.dt *= dt_coeff;
        };

        // increment forward with 5th order solution
        self.step_states(delta5);
        self.step_time(&dt_used);
        self.update_derivs();
        // dbg!(self.state.time);
        // dbg!(self.t_report[self.state.i]);
        dt_used
    }

    fn rk45_cash_karp_step(&mut self, dt: f64) -> (Vec<f64>, Vec<f64>) {
        self.update_derivs();

        // k1 = f(x_i, y_i)
        let k1s = self.derivs();

        // k2 = f(x_i + 1 / 5 * h, y_i + 1 / 5 * k1 * h)
        let mut sys1 = self.bare_clone();
        sys1.step_states_by_dt(&(dt / 5.));
        sys1.update_derivs();
        let k2s = sys1.derivs();

        // k3 = f(x_i + 3 / 10 * h, y_i + 3 / 40 * k1 * h + 9 / 40 * k2 * h)
        let mut sys2 = self.bare_clone();
        sys2.step_time(&(dt * 3. / 10.));
        sys2.step_states(
            k1s.iter()
                .zip(k2s.clone())
                .map(|(k1, k2)| (3. / 40. * k1 + 9. / 40. * k2) * dt)
                .collect(),
        );
        sys2.update_derivs();
        let k3s = sys2.derivs();

        // k4 = f(x_i + 3 / 5 * h, y_i + 3 / 10 * k1 * h - 9 / 10 * k2 * h + 6 / 5 * k3 * h)
        let mut sys3 = self.bare_clone();
        sys3.step_time(&(dt * 3. / 5.));
        sys3.step_states({
            let (k1s, k2s, k3s) = (k1s.clone(), k2s.clone(), k3s.clone());
            let zipped = zip!(k1s, k2s, k3s);
            let mut steps = vec![];
            for (k1, (k2, k3)) in zipped {
                steps.push((3. / 10. * k1 - 9. / 10. * k2 + 6. / 5. * k3) * dt);
            }
            steps
        });
        sys3.update_derivs();
        let k4s = sys3.derivs();

        // k5 = f(x_i + h, y_i - 11 / 54 * k1 * h + 5 / 2 * k2 * h - 70 / 27 * k3 * h + 35 / 27 * k4 * h)
        let mut sys4 = self.bare_clone();
        sys4.step_time(&dt);
        sys4.step_states({
            let (k1s, k2s, k3s, k4s) = (k1s.clone(), k2s.clone(), k3s.clone(), k4s.clone());
            let zipped = zip!(k1s, k2s, k3s, k4s);
            let mut steps = vec![];
            for (k1, (k2, (k3, k4))) in zipped {
                steps.push((-11. / 54. * k1 + 5. / 2. * k2 - 70. / 27. * k3 + 35. / 27. * k4) * dt);
            }
            steps
        });
        sys4.update_derivs();
        let k5s = sys4.derivs();

        // k6 = f(x_i + 7 / 8 * h, y_i + 1631 / 55296 * k1 * h + 175 / 512 * k2 * h + 575 / 13824 * k3 * h + 44275 / 110592 * k4 * h + 253 / 4096 * k5 * h)
        let mut sys5 = self.bare_clone();
        sys5.step_time(&(dt * 7. / 8.));
        sys5.step_states({
            let (k1s, k2s, k3s, k4s, k5s) = (
                k1s.clone(),
                k2s.clone(),
                k3s.clone(),
                k4s.clone(),
                k5s.clone(),
            );
            let zipped = zip!(k1s, k2s, k3s, k4s, k5s);
            let mut steps = vec![];
            for (k1, (k2, (k3, (k4, k5)))) in zipped {
                steps.push(
                    (1_631. / 55_296. * k1
                        + 175. / 512. * k2
                        + 575. / 13_824. * k3
                        + 44_275. / 110_592. * k4
                        + 253. / 4096. * k5)
                        * dt,
                );
            }
            steps
        });
        sys5.update_derivs();
        let k6s = sys5.derivs();

        // 4th order delta
        let mut delta4: Vec<f64> = vec![];
        // 5th order delta
        let mut delta5: Vec<f64> = vec![];
        let zipped = zip!(k1s, k2s, k3s, k4s, k5s, k6s);
        for (k1, (_k2, (k3, (k4, (k5, k6))))) in zipped {
            delta5.push(
                (37. / 378. * k1 + 250. / 621. * k3 + 125. / 594. * k4 + 512. / 1_771. * k6) * dt,
            );
            delta4.push(
                (2825. / 27_648. * k1
                    + 18_575. / 48_384. * k3
                    + 13_525. / 55_296. * k4
                    + 277. / 14_336. * k5
                    + 1. / 4. * k6)
                    * dt,
            );
        }
        (delta4, delta5)
    }
}
