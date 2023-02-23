use crate::imports::*;

/// ThermalMass component with capacitance, state, and history
#[derive(HistoryMethods, BareClone)]
#[common_derives]
#[pyo3_api(
    #[new]
    /// New thermal mass with capacitance `c` and initial temperature `t0`
    pub fn __new__(c: f64, temp0: f64) -> Self {
        Self::new(c, temp0)
    }
)]
pub struct ThermalMass {
    /// thermal capacitance \[J/K\]
    pub c: f64,
    pub state: ThermalMassState,
    pub history: ThermalMassStateHistoryVec,
}

impl ThermalMass {
    /// New thermal mass with capacitance `c` and initial temperature `t0`
    pub fn new(c: f64, temp0: f64) -> Self {
        Self {
            c,
            state: ThermalMassState {
                temp: temp0,
                dtemp: Default::default(),
            },
            history: Default::default(),
        }
    }
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

/// State for tracking temperature of [ThermalMass]
#[derive(Copy, HistoryVec)]
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
        Self::new(h)
    }
)]
#[common_derives]
pub struct Conductance {
    /// Thermal conductance \[W/K\] between two temperatures
    pub h: f64,
    pub state: ConductanceState,
    pub history: ConductanceStateHistoryVec,
}

impl Conductance {
    pub fn new(h: f64) -> Self {
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
#[derive(Copy, HistoryVec)]
#[common_derives]
#[pyo3_api]
pub struct ConductanceState {
    /// Heat transfer rate \[W\]
    pub q: f64,
}
