use proc_macros::*;
mod imports;
use imports::*;
mod solver;
mod traits;
use solver::*;
mod components;
use components::*;

// TODO:
// - make it so that q gets set with setter
// - make it so that temp gets set with setter
// - the above should make this connector domain agnostic

/// assumes heat flow from source -> sink is positive
/// calculates flow variable value first then updates states.
#[macro_export]
macro_rules! connect_states {
    ($sys: ident, ($($s0: ident, $s1: ident, $c: ident), +), $dt: ident) => {
        // update flow variables
        $(
            $sys.$c.state.q = $sys.$c.h * ($sys.$s0.state.pot() - $sys.$s1.state.pot());
        )+
        // update state variables
        $(
            $sys.$s0.state.temp -= $sys.$c.state.q * $dt / $sys.$s0.c;
            $sys.$s1.state.temp += $sys.$c.state.q * $dt / $sys.$s1.c;
        )+
    };
}

#[derive(Debug, Clone, PartialEq, PartialOrd, HistoryMethods)]
pub struct System {
    // components
    #[has_state]
    pub m1: ThermalMass,
    #[has_state]
    pub m2: ThermalMass,
    /// h12 connects m1 to m2
    #[has_state]
    pub h12: Conductance,
    #[has_state]
    pub m3: ThermalMass,
    #[has_state]
    pub h13: Conductance,

    // boiler plate fields (could be generated with proc macro)
    pub state: SystemState,
    pub history: SystemStateHistoryVec,
}

impl System {
    pub fn new(
        m1: ThermalMass,
        m2: ThermalMass,
        h12: Conductance,
        m3: ThermalMass,
        h13: Conductance,
    ) -> Self {
        Self {
            m1,
            m2,
            h12,
            m3,
            h13,
            state: Default::default(),
            history: Default::default(),
        }
    }
    pub fn step(&mut self, dt: f64) {
        connect_states!(self, (m1, m2, h12, m2, m3, h13), dt);
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

fn main() {
    let dt = 0.1;
    let m1 = ThermalMass::new(1.0, 0.0);
    let m2 = ThermalMass::new(2.0, 10.0);
    let h12 = Conductance::new(5.0, None);
    let m3 = ThermalMass::new(1.5, 12.0);
    let h13 = Conductance::new(5.0e-2, None);

    let mut system = System::new(m1, m2, h12, m3, h13);

    system.walk(Solver::FixedEuler { dt }, 2.0);

    dbg!(system);
}
