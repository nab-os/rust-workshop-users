// Replace each todo!() with a real implementation.
// cargo run --bin ex03_control_flow

// Number of steps to reach 1: if n is even, n / 2, otherwise 3n + 1.
// Try a `while` loop.
fn collatz_steps(n: u64) -> u32 {
    todo!()
}

// 90 and above is 'A', 75 and above 'B', 50 and above 'C', otherwise 'F'.
// Try a `match` with ranges like `90..=100`.
fn grade(score: u32) -> char {
    todo!()
}

// `&[i32]` is a slice, a view into a list of numbers. More on it this afternoon.
// Try a `for` loop.
fn count_positives(values: &[i32]) -> usize {
    todo!()
}

fn main() {
    assert_eq!(collatz_steps(1), 0);
    assert_eq!(collatz_steps(6), 8);
    assert_eq!(collatz_steps(27), 111);

    assert_eq!(grade(95), 'A');
    assert_eq!(grade(75), 'B');
    assert_eq!(grade(60), 'C');
    assert_eq!(grade(12), 'F');

    assert_eq!(count_positives(&[3, -1, 0, 8, -5, 2]), 3);
    assert_eq!(count_positives(&[]), 0);

    println!("ex03 done!");
}
