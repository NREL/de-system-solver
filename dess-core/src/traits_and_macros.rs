use crate::imports::*;
use crate::solver::AdaptiveSolverConfig;
use bincode::{deserialize, serialize};
use std::ffi::OsStr;
use std::fs::File;
use std::path::{Path, PathBuf};

/// zips multiple vectors into iterators
/// https://stackoverflow.com/a/62016977/941031
#[macro_export]
macro_rules! zip {
    ($x: expr) => ($x);
    ($x: expr, $($y: expr), +) => (
        $x.iter().zip(
            zip!($($y), +))
    )
}

/// assumes heat flow from source -> sink is positive
/// sets flow variable values
#[macro_export]
macro_rules! connect_states {
    ($sys: ident, $(($s0: ident, $s1: ident, $c: ident)), +) => {
        $(
            $sys.$c.set_flow(&$sys.$s0, &$sys.$s1);
        )+
    };
}

/// sets time derivatives of state variables based on connected flow variables
#[macro_export]
macro_rules! update_derivs {
    ($sys: ident, $(($s0: ident, $s1: ident, $c: ident)), +) => {
        $(
            $sys.$s0.step_deriv(-$sys.$c.flow() / $sys.$s0.c);
            $sys.$s1.step_deriv($sys.$c.flow() / $sys.$s1.c);
        )+
    };
}

pub trait HasState {
    /// sets value `val` of potential variable (e.g. temperature, pressure, voltage)
    fn set_state(&mut self, val: f64);
    /// returns value of potential variable (e.g. temperature, pressure, voltage)
    fn state(&self) -> f64;
    /// increments value of potential variable by multiplying `dt * self.derive()`
    /// and adding to previous value
    fn step_state_by_dt(&mut self, dt: &f64) {
        self.set_state(self.state() + dt * self.deriv());
    }
    /// increments value of potential by multiplying `dt * self.derive()`
    fn step_state(&mut self, val: f64) {
        self.set_state(self.state() + val);
    }
    /// sets value `val` of time derivative of potential variable
    fn set_deriv(&mut self, val: f64);
    /// returns value of time derivative of potential variable
    fn deriv(&self) -> f64;
    /// incremenents value of time derivative of pontental variable
    fn step_deriv(&mut self, val: f64) {
        self.set_deriv(self.deriv() + val)
    }
    /// returns value of storage variable (e.g. thermal capacitance \[J/K\])
    fn storage(&self) -> f64;
}

pub trait Flow {
    /// Sets flow variable based on difference between two potential variables
    fn set_flow(&mut self, p0: &dyn HasState, p1: &dyn HasState);
    /// returns value of flow variable (e.g. heat transfer, fluid flow rate, electrical current)
    fn flow(&self) -> f64;
}

pub trait Diff {
    fn diff(&self) -> Vec<f64>;
}

impl Diff for Vec<f64> {
    fn diff(&self) -> Vec<f64> {
        self.windows(2)
            .map(|vs| {
                let [x, y] = vs else {unreachable!()};
                y - x
            })
            .collect()
    }
}

pub trait SerdeAPI: Serialize + for<'a> Deserialize<'a> {
    #[allow(clippy::wrong_self_convention)]
    /// Save current data structure to file. Method adaptively calls serialization methods
    /// dependent on the suffix of the file given as str.
    ///
    /// # Argument:
    ///
    /// * `filename`: a `str` storing the targeted file name. Currently `.json` and `.yaml` suffixes are
    /// supported
    ///
    /// # Returns:
    ///
    /// A Rust Result
    fn to_file(&self, filename: &str) -> Result<(), anyhow::Error> {
        let file = PathBuf::from(filename);
        let extension = Path::new(filename)
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or("");
        let res = match extension {
            "json" => {
                serde_json::to_writer(&File::create(file)?, self)?;
                Ok(())
            }
            "yaml" => {
                serde_yaml::to_writer(&File::create(file)?, self)?;
                Ok(())
            }
            _ => Err(anyhow!("Unsupported file extension {}", extension)),
        };
        res
    }

    /// Read from file and return instantiated struct. Method adaptively calls deserialization
    /// methods dependent on the suffix of the file name given as str.
    /// Function returns a dynamic Error Result if it fails.
    ///
    /// # Argument:
    ///
    /// * `filename`: a `str` storing the targeted file name. Currently `.json` and `.yaml` suffixes are
    /// supported
    ///
    /// # Returns:
    ///
    /// A Rust Result wrapping data structure if method is called successfully; otherwise a dynamic
    /// Error.
    fn from_file(filename: &str) -> Result<Self, anyhow::Error>
    where
        Self: std::marker::Sized,
        for<'de> Self: Deserialize<'de>,
    {
        let extension = Path::new(filename)
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or("");

        let file = File::open(filename)?;
        match extension {
            "yaml" => Ok(serde_yaml::from_reader(file)?),
            "json" => Ok(serde_json::from_reader(file)?),
            _ => Err(anyhow!("Unsupported file extension {}", extension)),
        }
    }

    /// json serialization method.
    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    /// json deserialization method.
    fn from_json(json_str: &str) -> Result<Self, anyhow::Error> {
        Ok(serde_json::from_str(json_str)?)
    }

    /// yaml serialization method.
    fn to_yaml(&self) -> String {
        serde_yaml::to_string(&self).unwrap()
    }

    /// yaml deserialization method.
    fn from_yaml(yaml_str: &str) -> Result<Self, anyhow::Error> {
        Ok(serde_yaml::from_str(yaml_str)?)
    }

    /// bincode serialization method.
    fn to_bincode(&self) -> Vec<u8> {
        serialize(&self).unwrap()
    }

    /// bincode deserialization method.
    fn from_bincode(encoded: &[u8]) -> Result<Self, anyhow::Error> {
        Ok(deserialize(encoded)?)
    }
}

impl<T> SerdeAPI for T where T: Serialize + for<'a> Deserialize<'a> {}

pub trait Linspace {
    fn linspace(start: f64, stop: f64, n_elements: usize) -> Vec<f64> {
        let n_steps = n_elements - 1;
        let step_size = (stop - start) / n_steps as f64;
        let v_norm: Vec<f64> = (0..=n_steps)
            .collect::<Vec<usize>>()
            .iter()
            .map(|x| *x as f64)
            .collect();
        let v = v_norm.iter().map(|x| (x * step_size) + start).collect();
        v
    }
}

impl Linspace for Vec<f64> {}

pub trait BareClone {
    fn bare_clone(&self) -> Self;
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
