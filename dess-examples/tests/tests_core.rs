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
///function that compares two sets of values and returns true if the absolute error is within
/// specified epsilon
pub fn within_epsilon_absolute_error_only(comparison_vec: Vec<(&f64, &f64)>, epsilon: f64) -> bool {
    let mut close = vec![];
    for item in comparison_vec {
        if (item.0 - item.1).abs() < epsilon {
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
///compares two sets of values and returns the average distance between the two (in absolute value)
pub fn average_distance(comparison_vec: Vec<(&f64, &f64)>) -> f64 {
    let mut sum = 0.;
    let mut index = 0.;
    //not sure why this clone is needed, but without it, comparison_vec.is_empty() doesn't compile
    let comparison_vec_1 = comparison_vec.clone();
    for item in comparison_vec {
        sum += (item.0 - item.1).abs();
        index += 1.;
    }
    if comparison_vec_1.is_empty() {
        unreachable!("comparison_vec needs to be nonempty")
    } else {
        sum / index
    }
}
