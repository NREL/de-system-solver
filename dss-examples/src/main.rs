use dss_core::prelude::*;

pub mod imports;
use imports::*;
pub mod components;
pub use components::*;

/// System of connected components
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
    HistoryMethods,
    Walk,
    GetStateValues,
    BareClone,
)]
pub struct System {
    // components
    // the `has_state` attribute tells the Walk
    #[has_state]
    pub m1: ThermalMass,
    #[has_state]
    pub m2: ThermalMass,
    /// h12 connects m1 to m2
    #[history]
    pub h12: Conductance,
    #[has_state]
    pub m3: ThermalMass,
    #[history]
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
        connect_states!(self, (m1, m2, h12, m1, m3, h13), dt);
        update_states!(self, (m1, m2, h12, m1, m3, h13), dt);
        self.state.time += dt;
        self.save_state();
    }
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, HistoryVec,
)]
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
    let h13 = Conductance::new(5.0, None);

    let mut system = System::new(m1, m2, h12, m3, h13);

    system.walk(SolverOptions::FixedEuler { dt }, 2.0);

    system.to_file("temp_results.json").unwrap();

    // TODO: make a test around this
    // dbg!(system.bare_clone());
}
