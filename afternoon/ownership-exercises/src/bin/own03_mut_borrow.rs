// Does not compile. Fix it by reordering lines, not by adding .clone().
// Why would this be dangerous in C++? (Hint: what happens to a Vec's memory
// when it grows?)
// cargo run --bin own03_mut_borrow

fn add_reading(readings: &mut Vec<f64>, value: f64) {
    readings.push(value);
}

fn main() {
    let mut readings = vec![0.41, 0.39];

    let first = &readings[0];
    add_reading(&mut readings, 0.40);
    println!("first reading: {first}");

    assert_eq!(readings.len(), 3);
}
