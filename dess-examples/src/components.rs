use crate::imports::*;

/// ThermalMass component with capacitance, state, and history
#[derive(HistoryMethods, BareClone, Default)]
#[common_derives]
#[pyo3_api(
    #[new]
    /// New thermal mass with capacitance `c` and initial temperature `t0`
    pub fn __new__(c: f64, temp0: f64) -> Self {
        Self {
            c,
            state: ThermalMassState {
                temp: temp0,
                dtemp: Default::default(),
            },
            history: Default::default(),
        }
    }
)]
pub struct ThermalMass {
    /// thermal capacitance \[J/K\]
    pub c: f64,
    pub state: ThermalMassState,
    pub history: ThermalMassStateHistoryVec,
}

impl HasState for ThermalMass {
    fn set_state(&mut self, val: f64) {
        self.state.temp = val;
    }
    fn state(&self) -> f64 {
        self.state.temp
    }
    fn deriv(&self) -> f64 {
        self.state.dtemp
    }
    fn set_deriv(&mut self, val: f64) {
        self.state.dtemp = val;
    }
    fn step_deriv(&mut self, val: f64) {
        self.state.dtemp += val;
    }
    fn storage(&self) -> f64 {
        self.c
    }
}

/// ThermalReservoir component with capacitance, state, and history
#[derive(HistoryMethods, BareClone, Default)]
#[common_derives]
#[pyo3_api(
    #[new]
    /// New thermal reservoir with initial temperature `t0`
    pub fn __new__(temp0: f64) -> Self {
        Self {
            state: ThermalMassState {
                temp: temp0,
                dtemp: Default::default(),
            },
            history: Default::default(),
        }
    }
)]
pub struct ThermalReservoir {
    pub state: ThermalMassState,
    pub history: ThermalMassStateHistoryVec,
}

impl HasState for ThermalReservoir {
    fn set_state(&mut self, val: f64) {
        self.state.temp = val;
    }
    fn state(&self) -> f64 {
        self.state.temp
    }
    fn deriv(&self) -> f64 {
        self.state.dtemp
    }
    fn set_deriv(&mut self, _val: f64) {
        self.state.dtemp = 0.0;
    }
    fn step_deriv(&mut self, val: f64) {
        self.state.dtemp += val;
    }
    fn storage(&self) -> f64 {
        f64::INFINITY
    }
}

/// State for tracking temperature of [ThermalMass]
#[derive(Copy, HistoryVec, Default)]
#[common_derives]
#[pyo3_api]
pub struct ThermalMassState {
    /// temperature \[°C\]
    pub temp: f64,
    /// derivative of temperature w.r.t. time \[°C/s\]
    pub dtemp: f64,
}

/// Conductance component
#[derive(HistoryMethods, BareClone)]
#[pyo3_api(
    #[new]
    fn __new__(h: f64) -> Self {
        Self {
            h,
            state: ConductanceState {
                q: Default::default(),
            },
            history: ConductanceStateHistoryVec {
                q: Default::default(),
            },
        }
    }
)]
#[common_derives]
#[derive(Default)]
pub struct Conductance {
    /// Thermal conductance \[W/K\] between two temperatures
    pub h: f64,
    pub state: ConductanceState,
    pub history: ConductanceStateHistoryVec,
}

impl Flow for Conductance {
    fn flow(&self) -> f64 {
        self.state.q
    }
    fn set_flow(&mut self, p0: &dyn HasState, p1: &dyn HasState) {
        self.state.q = self.h * (p0.state() - p1.state());
    }
}

/// Struct for tracking flow variables in Conductance
#[derive(Copy, HistoryVec, Default)]
#[common_derives]
#[pyo3_api]
pub struct ConductanceState {
    /// Heat transfer rate \[W\]
    pub q: f64,
}
