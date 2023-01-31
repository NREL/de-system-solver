use crate::imports::*;

/// ThermalMass component with capacitance, state, and history
#[derive(Debug, Clone, PartialEq, PartialOrd, HistoryMethods)]
pub struct ThermalMass {
    /// thermal capacitance \[J/K\]
    pub c: f64,
    pub state: ThermalMassState,
    pub history: ThermalMassStateHistoryVec,
}

impl ThermalMass {
    /// New thermal mass with capacitance `c` and initial temperature `t0`
    pub fn new(c: f64, t0: f64) -> Self {
        Self {
            c,
            state: ThermalMassState { temp: t0 },
            history: ThermalMassStateHistoryVec { temp: vec![t0] },
        }
    }
}

/// State for tracking temperature of [ThermalMass]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, HistoryVec)]
pub struct ThermalMassState {
    /// temperature \[°C\]
    pub temp: f64,
}

impl Potential for ThermalMassState {
    fn set_pot(&mut self, val: f64) {
        self.temp = val
    }
    fn pot(&self) -> f64 {
        self.temp
    }
}

/// Conductance component
#[derive(Debug, Clone, PartialEq, PartialOrd, HistoryMethods)]
pub struct Conductance {
    /// Thermal conductance \[W/K\] between two temperatures
    pub h: f64,
    pub state: ConductanceState,
    pub history: ConductanceStateHistoryVec,
}

impl Conductance {
    pub fn new(h: f64, q0: Option<f64>) -> Self {
        Self {
            h,
            state: ConductanceState {
                q: q0.unwrap_or_default(),
            },
            history: ConductanceStateHistoryVec {
                q: vec![q0.unwrap_or_default()],
            },
        }
    }
}

impl Flow for Conductance {
    fn flow(&self) -> f64 {
        self.state.q
    }
    fn set_flow(&mut self, p0: &dyn Potential, p1: &dyn Potential) {
        self.state.q = self.h * (p0.pot() - p1.pot());
    }
}

/// Struct for tracking flow variables in Conductance
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, HistoryVec)]
pub struct ConductanceState {
    /// Heat transfer rate \[W\]
    pub q: f64,
}
