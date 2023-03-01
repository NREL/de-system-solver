use crate::imports::*;

#[common_derives]
pub enum SolverOptions {
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

impl Default for SolverOptions {
    fn default() -> Self {
        SolverOptions::RK4Fixed { dt: 0.1 }
    }
}

#[pyo3_api(
    #[new]
    fn new_py(
        dt_init: f64,
        dt_max: Option<f64>,
        max_iter: Option<u32>,
        tol: Option<f64>,
        save: Option<bool>,
    ) -> Self {
        let mut solver = Self::default();
        solver.state.dt_prev = dt_init;
        if let Some(dt_max) = dt_max {
            solver.dt_max = dt_max;
        }
        if let Some(max_iter) = max_iter {
            solver.max_iter = max_iter;
        }
        if let Some(tol) = tol {
            solver.tol = tol;
        }
        if let Some(save) = save {
            solver.save = save;
        }
        solver
    }
)]
#[common_derives]
#[derive(HistoryMethods)]
pub struct AdaptiveSolverConfig {
    /// max allowable dt
    pub dt_max: f64,
    /// max number of iterations per time step
    pub max_iter: u32,
    /// euclidean error tolerance
    pub tol: f64,
    /// save iteration history
    pub save: bool,
    /// current iteration variables
    pub state: SolverState,
    /// iteration history
    pub history: SolverStateHistoryVec,
}

impl AdaptiveSolverConfig {
    pub fn new(dt_init: f64, dt_max: f64, max_iter: u32, tol: f64, save: bool) -> Self {
        Self {
            dt_max,
            max_iter,
            tol,
            save,
            state: SolverState {
                dt_prev: dt_init,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl Default for AdaptiveSolverConfig {
    fn default() -> Self {
        Self {
            dt_max: 1.0,
            max_iter: 2,
            tol: 1e-6,
            save: false,
            state: Default::default(),
            history: Default::default(),
        }
    }
}

#[common_derives]
#[pyo3_api]
#[derive(HistoryVec, Copy)]
pub struct SolverState {
    /// time step size in previous interval
    pub dt_prev: f64,
    /// number of iterations to achieve tolerance
    pub n_iters: u8,
    /// L2 (euclidean) norm
    pub norm: f64,
    /// current system time used in solver
    pub t_curr: f64,
}

impl Default for SolverState {
    fn default() -> Self {
        Self {
            dt_prev: 0.1,
            n_iters: 0,
            norm: 0.0,
            t_curr: 0.0,
        }
    }
}


pub trait SolverBase: BareClone + Sized {
    /// assuming `set_derivs` has been called, steps
    /// value of states by deriv * dt
    fn step_by_dt(&mut self, dt: &f64);

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
    }

    /// solves time step with adaptive Cash-Karp Method (variant of RK45) and returns `dt` used
    /// https://en.wikipedia.org/wiki/Cash%E2%80%93Karp_method
    fn rk45_cash_karp(&mut self, dt_max: &f64, sol: AdaptiveSolverConfig) -> f64 {
        let dt = dt_max.min(sol.state.dt_prev);
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

        // increment forward with 5th order solution
        self.step(delta5);

        dt.clone()
    }
}
