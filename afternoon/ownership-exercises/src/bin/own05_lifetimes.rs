// Bonus. Does not compile: the compiler cannot tell whether the returned
// reference comes from `a` or from `b`. Add a lifetime parameter to fix it,
// then explain why the commented block in main would be rejected.
// cargo run --bin own05_lifetimes

fn longest(a: &str, b: &str) -> &str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    let control = String::from("control");
    let sample = String::from("A1");
    assert_eq!(longest(&control, &sample), "control");

    // let result;
    // {
    //     let short_lived = String::from("a much longer name");
    //     result = longest(&control, &short_lived);
    // }
    // println!("{result}");

    println!("own05 done!");
}
