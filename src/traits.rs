pub trait Potential {
    /// sets value `val` of potential variable (e.g. temperature, pressure, voltage)
    fn set_pot(&mut self, val: f64);
    /// returns value of potential variable (e.g. temperature, pressure, voltage)
    fn pot(&self) -> f64;
    /// increments value of potential by `val`
    fn step_pot(&mut self, val: f64) {
        self.set_pot(self.pot() + val);
    }
}

pub trait Flow {
    /// Sets flow variable based on difference between two potential variables
    fn set_flow(&mut self, p0: &dyn Potential, p1: &dyn Potential);
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
