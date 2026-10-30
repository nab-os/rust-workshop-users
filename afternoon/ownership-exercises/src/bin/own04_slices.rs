// Replace each todo!() with a real implementation.
// cargo run --bin own04_slices

// The largest value, None if the slice is empty.
fn largest(values: &[f64]) -> Option<f64> {
    todo!()
}

// The part of `line` before the first comma, or the whole line if there is
// none. It must not allocate a new String: return a slice of `line`.
// Hint: line.find(',') and &line[..i]
fn first_field(line: &str) -> &str {
    todo!()
}

// Multiply every value in place.
fn scale(values: &mut [f64], factor: f64) {
    todo!()
}

fn main() {
    let mut values = vec![0.4, 1.3, 0.05, 2.0];
    assert_eq!(largest(&values), Some(2.0));
    assert_eq!(largest(&values[..2]), Some(1.3));
    assert_eq!(largest(&[]), None);

    assert_eq!(first_field("A1,0.412"), "A1");
    assert_eq!(first_field("no comma"), "no comma");

    scale(&mut values, 10.0);
    assert_eq!(values, vec![4.0, 13.0, 0.5, 20.0]);

    println!("own04 done!");
}
