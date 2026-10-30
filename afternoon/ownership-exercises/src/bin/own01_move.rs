// Does not compile. Read the error message carefully, then fix it in two
// different ways: once with .clone(), once by borrowing with &.
// cargo run --bin own01_move

fn main() {
    let sample = String::from("A1");
    let label = sample;

    println!("sample = {sample}");
    println!("label = {label}");

    // Why does this one work without any change?
    let count = 3;
    let copy = count;
    println!("{count} {copy}");
}
