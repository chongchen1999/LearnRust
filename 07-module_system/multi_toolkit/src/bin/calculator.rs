use multi_toolkit::math::algebra;

fn main() {
    let a = 5.0;
    let b = -10.0;

    match algebra::solve_linear(a, b) {
        Some(solution) => println!("Solution to {}x + {} = 0 is x = {}", a, b, solution),
        None => println!("No solution exists."),
    }
}
