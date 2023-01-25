use proc_macros::*;
use std::vec;

mod imports;
use imports::*;
mod traits;
// use traits::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, HistoryMethods)]
pub struct ThermalMass {
    /// thermal capacitance
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, HistoryVec)]
pub struct ThermalMassState {
    /// temperature \[°C\]
    pub temp: f64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, HistoryMethods)]
pub struct Conductance {
    /// Thermal conductance between two temperatures
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

/// Struct for tracking flow variables in Conductance
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, HistoryVec)]
pub struct ConductanceState {
    pub q: f64,
}

/// this is limited to only two thermal masses as currently coded.
/// TODO:
/// - figure out how to abstract connector to be direction agnostic
///   or include heat flows inside masses and not connector
/// - make it so that q gets set with setter
/// - make it so that temp gets set with setter
/// assumes heat flow from source -> sink is positive
/// calculates flow variable value first then updates states.
macro_rules! connect_heat {
    ($source: expr, $sink: expr, $connector: expr, $dt: expr) => {
        $connector.state.q = $connector.h * ($source.state.temp - $sink.state.temp);
        $source.state.temp += -$connector.state.q * $dt / $source.c;
        $sink.state.temp += $connector.state.q * $dt / $sink.c;
    };
}

#[derive(Debug, Clone, PartialEq, PartialOrd, HistoryMethods)]
pub struct System {
    #[has_state]
    pub m1: ThermalMass,
    #[has_state]
    pub m2: ThermalMass,
    #[has_state]
    pub h12: Conductance,
    pub state: SystemState,
    pub history: SystemStateHistoryVec,
}

impl System {
    pub fn new(m1: ThermalMass, m2: ThermalMass, h12: Conductance) -> Self {
        Self {
            m1,
            m2,
            h12,
            state: Default::default(),
            history: Default::default(),
        }
    }
    pub fn step(&mut self, dt: f64) {
        connect_heat!(self.m1, self.m2, self.h12, dt);
        self.state.time += dt;
        self.save_state();
    }

    pub fn walk(&mut self, solver: Solver, end_time: f64) {
        match solver {
            Solver::FixedEuler { dt } => {
                while self.state.time < end_time {
                    self.step(dt)
                }
            }
            _ => todo!(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, HistoryVec)]
pub struct SystemState {
    // current time
    time: f64,
}

pub enum Solver {
    FixedEuler { dt: f64 },
    ToDo,
}

fn main() {
    let dt = 0.1;
    let m1 = ThermalMass::new(1.0, 0.0);
    let m2 = ThermalMass::new(2.0, 10.0);
    let h12 = Conductance::new(5.0, None);

    let mut system = System::new(m1, m2, h12);

    system.walk(Solver::FixedEuler { dt }, 2.0);

    dbg!(system.h12.history);
    dbg!(system.m1.history);
    dbg!(system.m2.history);
}
