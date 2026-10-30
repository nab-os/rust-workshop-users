// Replace each todo!() with a real implementation.
// cargo run --bin ex04_structs_enums

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn square(size: f64) -> Rectangle {
        todo!()
    }

    fn area(&self) -> f64 {
        todo!()
    }

    fn is_square(&self) -> bool {
        todo!()
    }
}

#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rect(Rectangle),
    Triangle { base: f64, height: f64 },
}

// Use `match` and reuse Rectangle::area for the Rect case.
fn shape_area(shape: &Shape) -> f64 {
    todo!()
}

fn main() {
    let r = Rectangle {
        width: 3.0,
        height: 4.0,
    };
    assert_eq!(r.area(), 12.0);
    assert!(!r.is_square());

    let s = Rectangle::square(2.0);
    assert!(s.is_square());
    println!("{s:?}");

    let shapes = [
        Shape::Circle { radius: 1.0 },
        Shape::Rect(r),
        Shape::Triangle {
            base: 6.0,
            height: 2.0,
        },
    ];
    let areas: Vec<f64> = shapes.iter().map(shape_area).collect();
    assert_eq!(areas, vec![std::f64::consts::PI, 12.0, 6.0]);
    println!("{shapes:#?}");

    println!("ex04 done!");
}
