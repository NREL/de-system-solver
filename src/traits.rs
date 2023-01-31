pub trait Potential {
    /// sets value of potential variable (e.g. temperature, pressure, voltage)
    fn set_pot(&mut self, val: f64);
    /// returns value of potential variable (e.g. temperature, pressure, voltage)
    fn pot(&self) -> f64;
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
