use std::vec;
use proc_macros::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ThermalMass {
    /// thermal capacitance
    pub c: f64,
    pub state: ThermalMassState,
    pub history: ThermalMassHistory,
}

impl ThermalMass {
    /// New thermal mass with capacitance `c` and initial temperature `t0`
    pub fn new(c: f64, t0: f64) -> Self {
        Self {
            c,
            state: ThermalMassState { t: t0 },
            history: ThermalMassHistory { t: vec![t0] },
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ThermalMassState {
    /// temperature \[°C\]
    pub t: f64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ThermalMassHistory {
    pub t: Vec<f64>,
}

impl ThermalMassHistory {
    // this can be automated with a procedural macro similar to ALTRIOS
    pub fn push(&mut self, state: ThermalMassState) {
        self.t.push(state.t)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Conductance {
    /// Thermal conductance between two temperatures
    pub h: f64,
    pub state: ConductanceState,
    pub history: ConductanceHistory,
}

impl Conductance {
    pub fn new(h: f64, q0: Option<f64>) -> Self {
        Self {
            h,
            state: ConductanceState {
                q: q0.unwrap_or_default(),
            },
            history: ConductanceHistory {
                q: vec![q0.unwrap_or_default()],
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ConductanceState {
    pub q: f64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ConductanceHistory {
    pub q: Vec<f64>,
}

impl ConductanceHistory {
    pub fn push(&mut self, state: ConductanceState) {
        self.q.push(state.q);
    }
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

fn main() {
    let time_step = 0.1;

    let time_vec: Vec<f64> = (0..=10)
        .collect::<Vec<i64>>()
        .iter()
        .map(|x| *x as f64 * time_step)
        .collect();

    let dt_vec: Vec<f64> = time_vec.diff();

    let mut m1 = ThermalMass::new(1.0, 0.0);
    let mut m2 = ThermalMass::new(2.0, 10.0);
    let mut h12 = Conductance::new(5.0, None);

    for (_, dt) in dt_vec.iter().enumerate() {
        // assumes heat flow from 1 -> 2 is positive
        h12.state.q = h12.h * (m1.state.t - m2.state.t);
        h12.history.push(h12.state.clone());

        m1.state.t += -h12.state.q * dt / m1.c;
        m1.history.push(m1.state.clone());

        m2.state.t += h12.state.q * dt / m2.c;
        m2.history.push(m2.state.clone());
    }

    dbg!(h12.history);
    dbg!(m1.history);
    dbg!(m2.history);
}
