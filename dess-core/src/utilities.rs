pub fn almost_eq(val1: f64, val2: f64, epsilon: Option<f64>) -> bool {
    let epsilon = epsilon.unwrap_or(1e-8);
    ((val2 - val1) / (val1 + val2)).abs() < epsilon || (val2 - val1).abs() < epsilon
}

