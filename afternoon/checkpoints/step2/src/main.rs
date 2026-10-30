use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug, Clone, PartialEq)]
struct Measurement {
    sample: String,
    value: f64,
}

fn parse_line(line: &str) -> Result<Measurement, String> {
    let (sample, value) = line
        .split_once(',')
        .ok_or(format!("expected `sample,value`, got `{line}`"))?;

    let sample = sample.trim();
    if sample.is_empty() {
        return Err("empty sample name".to_string());
    }

    let value: f64 = value
        .trim()
        .parse()
        .map_err(|e| format!("bad value `{}`: {e}", value.trim()))?;

    Ok(Measurement {
        sample: sample.to_string(),
        value,
    })
}

fn parse(text: &str) -> (Vec<Measurement>, Vec<String>) {
    let mut measurements = Vec::new();
    let mut errors = Vec::new();

    for (i, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        match parse_line(line) {
            Ok(m) => measurements.push(m),
            Err(e) => errors.push(format!("line {}: {e}", i + 1)),
        }
    }
    (measurements, errors)
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).ok_or("usage: labstats <file>")?;
    let text = fs::read_to_string(&path)?;

    let (measurements, errors) = parse(&text);
    for e in &errors {
        eprintln!("warning: {e}");
    }
    for m in &measurements {
        println!("{m:?}");
    }
    println!(
        "{} measurements, {} skipped",
        measurements.len(),
        errors.len()
    );
    Ok(())
}
