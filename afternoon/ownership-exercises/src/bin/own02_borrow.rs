// Does not compile. Change the signature of `describe` so that it borrows
// its arguments instead of taking ownership. Do not add any .clone().
// cargo run --bin own02_borrow

fn describe(name: String, values: Vec<f64>) -> String {
    format!("{name}: {} values", values.len())
}

fn main() {
    let name = String::from("B2");
    let values = vec![1.25, 1.31, 1.28];

    let first = describe(name, values);
    let second = describe(name, values);

    assert_eq!(first, second);
    println!("{first}");
}
