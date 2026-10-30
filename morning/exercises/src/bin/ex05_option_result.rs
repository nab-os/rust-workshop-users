// Replace each todo!() with a real implementation.
// cargo run --bin ex05_option_result

// None when dividing by zero.
fn safe_divide(a: f64, b: f64) -> Option<f64> {
    todo!()
}

// The first negative value, if there is one.
fn first_negative(values: &[i32]) -> Option<i32> {
    todo!()
}

// Parse a temperature in Celsius. Return an error message if the text is not
// a number, or if it is below absolute zero (-273.15).
// Hint: "21.5".parse::<f64>() returns a Result.
fn parse_celsius(text: &str) -> Result<f64, String> {
    todo!()
}

// Use parse_celsius and the `?` operator. Kelvin = Celsius + 273.15.
fn parse_kelvin(text: &str) -> Result<f64, String> {
    todo!()
}

fn main() {
    assert_eq!(safe_divide(1.0, 4.0), Some(0.25));
    assert_eq!(safe_divide(1.0, 0.0), None);

    assert_eq!(first_negative(&[4, 2, -7, -1]), Some(-7));
    assert_eq!(first_negative(&[1, 2, 3]), None);

    assert_eq!(parse_celsius("21.5"), Ok(21.5));
    assert_eq!(parse_celsius(" -10 "), Ok(-10.0));
    assert!(parse_celsius("warm").is_err());
    assert!(parse_celsius("-300").is_err());

    assert_eq!(parse_kelvin("0"), Ok(273.15));
    assert!(parse_kelvin("cold").is_err());

    match parse_celsius("-500") {
        Ok(t) => println!("parsed {t}"),
        Err(e) => println!("expected error: {e}"),
    }

    println!("ex05 done!");
}
