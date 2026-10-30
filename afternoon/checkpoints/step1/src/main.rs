use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).ok_or("usage: labstats <file>")?;
    let text = fs::read_to_string(&path)?;

    for (i, line) in text.lines().enumerate() {
        println!("{:>3} | {line}", i + 1);
    }
    println!("{} lines in {path}", text.lines().count());
    Ok(())
}
