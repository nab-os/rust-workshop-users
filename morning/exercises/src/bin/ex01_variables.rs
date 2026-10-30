// This file does not compile yet. Fix it, then make it run without panicking.
// cargo run --bin ex01_variables

fn main() {
    let count = 0;
    count += 1;
    assert_eq!(count, 1);

    let temperature = "21.5";
    let temperature: f64 = temperature;
    assert_eq!(temperature, 21.5);

    let x: i32 = 7;
    let y: f64 = 2.0;
    let ratio = x / y;
    assert_eq!(ratio, 3.5);

    const MAX_SAMPLES: u32 = 100;
    let samples: u8 = 300;
    assert!(samples as u32 <= MAX_SAMPLES * 3);

    let point = (1.5, -2.0);
    let (px, py) = point;
    assert_eq!(px + py, -0.5);

    println!("ex01 done!");
}
