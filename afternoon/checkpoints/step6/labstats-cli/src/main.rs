use std::env;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::process;

use labstats_core::{parse, parse_line, report};

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

fn run() -> Result<(), Box<dyn Error>> {
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

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        process::exit(1);
    }
}
