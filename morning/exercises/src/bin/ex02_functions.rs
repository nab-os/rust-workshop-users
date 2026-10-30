// Replace each todo!() with a real implementation.
// cargo run --bin ex02_functions

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    todo!()
}

fn is_even(n: i64) -> bool {
    todo!()
}

// 1 + 2 + ... + n
fn sum_to(n: u64) -> u64 {
    todo!()
}

// "Fizz" for multiples of 3, "Buzz" for multiples of 5, "FizzBuzz" for both,
// the number itself otherwise.
fn fizzbuzz(n: u32) -> String {
    todo!()
}

fn main() {
    assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
    assert_eq!(celsius_to_fahrenheit(-40.0), -40.0);

    assert!(is_even(4));
    assert!(!is_even(-3));

    assert_eq!(sum_to(0), 0);
    assert_eq!(sum_to(10), 55);

    assert_eq!(fizzbuzz(3), "Fizz");
    assert_eq!(fizzbuzz(10), "Buzz");
    assert_eq!(fizzbuzz(30), "FizzBuzz");
    assert_eq!(fizzbuzz(7), "7");

    println!("ex02 done!");
}
