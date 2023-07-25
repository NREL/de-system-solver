///compares two sets of values to see if they remain within epsilon of each other
///using relative error calculation of val1-val2 divided by the average of val1 and val2 (to avoid dividing by 0
///while maintaining similar range of values to regular relative error)
pub fn within_epsilon(comparison_vec: Vec<(&f64, &f64)>, epsilon: f64) -> bool {
    let mut close = vec![];
    for item in comparison_vec {
        if (2. * (item.0 - item.1) / (item.0 + item.1)).abs() < epsilon
            || (item.0 - item.1).abs() < epsilon
        {
            close.push(true);
        } else {
            close.push(false);
            break;
        }
    }
    if close.is_empty() {
        unreachable!("comparison_vec needs to be nonempty")
    } else {
        let length = close.len();
        close[length - 1]
    }
}