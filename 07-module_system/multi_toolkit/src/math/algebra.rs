pub fn solve_linear(a: f64, b: f64) -> Option<f64> {
    if a == 0.0 {
        None
    } else {
        Some(-b / a)
    }
}
