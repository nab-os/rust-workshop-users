use std::collections::BTreeMap;
use std::env;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::Write;

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

#[derive(Debug, PartialEq)]
struct Summary {
    count: usize,
    mean: f64,
    min: f64,
    max: f64,
}

fn group_by_sample(measurements: &[Measurement]) -> BTreeMap<&str, Vec<f64>> {
    let mut groups: BTreeMap<&str, Vec<f64>> = BTreeMap::new();
    for m in measurements {
        groups.entry(m.sample.as_str()).or_default().push(m.value);
    }
    groups
}

fn summarize(values: &[f64]) -> Option<Summary> {
    if values.is_empty() {
        return None;
    }
    let mut total = 0.0;
    let mut min = values[0];
    let mut max = values[0];
    for &v in values {
        total += v;
        if v < min {
            min = v;
        }
        if v > max {
            max = v;
        }
    }
    let count = values.len();
    let mean = total / count as f64;
    Some(Summary {
        count,
        mean,
        min,
        max,
    })
}

fn report(measurements: &[Measurement]) -> String {
    let mut out = String::from("sample\tcount\tmean\tmin\tmax\n");
    for (sample, values) in group_by_sample(measurements) {
        if let Some(s) = summarize(&values) {
            out.push_str(&format!(
                "{sample}\t{}\t{:.3}\t{:.3}\t{:.3}\n",
                s.count, s.mean, s.min, s.max
            ));
        }
    }
    out
}

const USAGE: &str = "usage:
    labstats summary <file> [output]
    labstats add <file> <sample> <value>";

fn summary(path: &str, output: Option<&str>) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let (measurements, errors) = parse(&text);
    for e in &errors {
        eprintln!("warning: {e}");
    }

    let table = report(&measurements);
    match output {
        Some(out) => {
            fs::write(out, &table)?;
            println!("report written to {out}");
        }
        None => print!("{table}"),
    }
    Ok(())
}

fn add(path: &str, sample: &str, value: &str) -> Result<(), Box<dyn Error>> {
    let m = parse_line(&format!("{sample},{value}"))?;
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{},{}", m.sample, m.value)?;
    println!("added {} = {} to {path}", m.sample, m.value);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        return Err(USAGE.into());
    }
    match args[0].as_str() {
        "summary" if args.len() == 2 => summary(&args[1], None),
        "summary" if args.len() == 3 => summary(&args[1], Some(&args[2])),
        "add" if args.len() == 4 => add(&args[1], &args[2], &args[3]),
        _ => Err(USAGE.into()),
    }
}
