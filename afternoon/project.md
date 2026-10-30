# Mini project: `labstats`

We'll build a small command line tool that reads a file of measurements,
computes statistics per sample, saves a report and records new measurements.
At the end we'll split it into a reusable library and a program in a cargo
workspace.

The input is `data/measurements.csv`:

```
# sample,value
A1,0.412
A1,0.398
B2,1.250
...
B2,oops
,0.500
```

Some lines are deliberately broken. Real data always is.

The finished tool will work like this:

```console
$ labstats summary data/measurements.csv
warning: line 12: bad value `oops`: invalid float literal
warning: line 17: empty sample name
sample  count   mean    min     max
A1      5       0.407   0.398   0.420
B2      4       1.284   1.250   1.310
C3      3       2.004   1.998   2.010
control 4       0.050   0.047   0.052

$ labstats add data/measurements.csv C3 2.001
added C3 = 2.001 to data/measurements.csv
```

**If you get lost**, copy the folder from `checkpoints/` for the previous step
and continue from there. Nobody needs to finish every step on their own.

---

## Warm-up: `ex05_option_result`

Before touching the project, do the last morning exercise:

```sh
cd morning/exercises
cargo run --bin ex05_option_result
```

Reading a file, parsing a number, finding a sample: everything in this
project can fail, so `Option` and `Result` are everywhere in it.

---

## Step 1: read a text file

```sh
cargo new labstats
cd labstats
mkdir data
cp ../data/measurements.csv data/      # adjust the path to where you put it
```

Goal: `cargo run -- data/measurements.csv` prints every line of the file with
its line number.

You'll need:

- `std::env::args()` gives the program arguments as an iterator.
  `.nth(1)` is the first real argument (`nth(0)` is the program name).
  It returns an `Option<String>`.
- `std::fs::read_to_string(path)` returns a `Result<String, std::io::Error>`.
- `text.lines()` iterates over lines, `.enumerate()` adds an index.

Start with `.expect("...")` to get the values out, run it with a wrong file
name, and look at the crash. Then improve it: make `main` return a `Result`
and use `?`.

```rust
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args().nth(1).ok_or("usage: labstats <file>")?;
    // ...
    Ok(())
}
```

`Box<dyn Error>` means "any kind of error". It's the easiest error type to
start with, and `?` will convert most errors into it for you.

Checkpoint: `checkpoints/step1`.

---

## Step 2: parse lines into a list of measurements

Goal: turn the text into a `Vec<Measurement>`, and collect a warning for
every bad line instead of crashing.

```rust
#[derive(Debug, Clone, PartialEq)]
struct Measurement {
    sample: String,
    value: f64,
}
```

Write two functions:

```rust
// "A1,0.412" -> Ok(Measurement { sample: "A1", value: 0.412 })
// "B2,oops"  -> Err("bad value `oops`: ...")
fn parse_line(line: &str) -> Result<Measurement, String> {
    todo!()
}

// Skips empty lines and lines starting with '#'.
// Returns the measurements and the error messages ("line 12: ...").
fn parse(text: &str) -> (Vec<Measurement>, Vec<String>) {
    todo!()
}
```

Useful methods:

- `line.split_once(',')` returns `Option<(&str, &str)>`
- `s.trim()`, `s.is_empty()`, `s.starts_with('#')`
- `s.parse::<f64>()` returns a `Result`
- `.ok_or(...)` turns an `Option` into a `Result`
- `.map_err(|e| format!("...{e}"))` changes the error of a `Result`
- `Vec::new()`, `v.push(x)`, `v.len()`
- `sample.to_string()` turns a `&str` into an owned `String`

In `main`, print each warning with `eprintln!` (to stderr) and each
measurement with `println!("{m:?}")`.

Questions to think about:

- Why does `Measurement` hold a `String` and not a `&str`?
- Why can `parse_line` take a `&str` instead of a `String`?

We'll answer them in the next step.

Checkpoint: `checkpoints/step2`.

---

## Step 3: memory, ownership and borrowing

A short talk, then exercises in `ownership-exercises`:

```sh
cd ownership-exercises
cargo run --bin own01_move
```

| Exercise | Topic |
|----------|-------|
| `own01_move` | Moving a value, `.clone()`, `Copy` types |
| `own02_borrow` | Borrowing with `&` instead of taking ownership |
| `own03_mut_borrow` | One `&mut` or many `&`, never both |
| `own04_slices` | `&[T]`, `&mut [T]` and `&str`: views into data owned elsewhere |
| `own05_lifetimes` | Bonus: lifetime annotations |

The three rules:

1. Every value has exactly one owner.
2. When the owner goes out of scope, the value is freed (no garbage
   collector, no `free()`).
3. You can borrow a value: either many shared references `&T`, or one
   mutable reference `&mut T`, and a reference can never outlive the value.

---

## Step 4: group and summarize

Goal: print one line of statistics per sample, sorted by sample name.

```rust
use std::collections::HashMap;

fn group_by_sample(measurements: &[Measurement]) -> HashMap<&str, Vec<f64>> {
    todo!()
}
```

Look at the return type: the keys are `&str` borrowed from the measurements,
so we don't copy any sample names. The compiler knows the map borrows from
`measurements`. Try calling `drop(measurements);` before printing the map and
read the error.

Hint: `map.entry(key).or_default().push(value)` inserts an empty `Vec` the
first time a key is seen, then pushes into it.

Then:

```rust
#[derive(Debug, PartialEq)]
struct Summary {
    count: usize,
    mean: f64,
    min: f64,
    max: f64,
}

// None for an empty slice.
fn summarize(values: &[f64]) -> Option<Summary> {
    todo!()
}
```

A `for` loop is enough: start `min` and `max` at `values[0]`, add every
value to a running total, and compare as you go. `values.len() as f64` for
the mean. (Iterators can do it in one line each, but there is no prize for
that today.)

Run the program twice. Is the order of the samples the same? Replace
`HashMap` with `BTreeMap` and run it again.

Finally, write `fn report(measurements: &[Measurement]) -> String` that builds
the whole table as a `String` (tab separated, three decimals: `{:.3}`). We
return a `String` instead of printing directly so that the next step can
write it to a file.

Checkpoint: `checkpoints/step4`.

---

## Step 5: save a report and record new measurements

Goal: two subcommands.

```sh
cargo run -- summary data/measurements.csv            # print the report
cargo run -- summary data/measurements.csv report.tsv # save it to a file
cargo run -- add data/measurements.csv C3 2.001       # append a measurement
```

- Writing a whole file: `std::fs::write(path, &text)?`
- Appending to a file:

  ```rust
  use std::fs::OpenOptions;
  use std::io::Write;

  let mut file = OpenOptions::new().create(true).append(true).open(path)?;
  writeln!(file, "{},{}", m.sample, m.value)?;
  ```

- Validate the new measurement with your `parse_line` before writing it, so
  `add ... C3 nope` is refused.
- Collect the arguments into a `Vec<String>` and match on the first one:

  ```rust
  let args: Vec<String> = std::env::args().skip(1).collect();
  if args.is_empty() {
      return Err(USAGE.into());
  }
  match args[0].as_str() {
      "summary" if args.len() == 2 => summary(&args[1], None),
      "summary" if args.len() == 3 => summary(&args[1], Some(&args[2])),
      "add" if args.len() == 4 => add(&args[1], &args[2], &args[3]),
      _ => Err(USAGE.into()),
  }
  ```

  `as_str()` turns the `String` into a `&str` so it can be compared with the
  literals. The `if` after a pattern is a guard: the arm only matches when
  it's true.

Run it without arguments. The usage message is printed in a strange way.
Why? We'll fix it in the next step.

Checkpoint: `checkpoints/step5`.

---

## Step 6: a cargo workspace

This step is shown as a demo at the end of the day. If we run out of time,
follow it at home from `checkpoints/step5`.

Goal: split the code into

- `labstats-core`, a **library** with `Measurement`, `parse_line`, `parse`,
  `group_by_sample`, `summarize`, `report`, and unit tests;
- `labstats-cli`, a **binary** that depends on the library and only handles
  arguments and files.

Someone else in the lab could then use `labstats-core` from their own program
without touching the command line part.

Create a folder `labstats-ws` with a `data/` folder (copy the CSV again) and
this `labstats-ws/Cargo.toml`:

```toml
[workspace]
resolver = "3"
members = []
```

Then create the two crates inside it:

```sh
cd labstats-ws
cargo new --vcs none --lib labstats-core
cargo new --vcs none labstats-cli
cat Cargo.toml
```

Cargo noticed the workspace and filled in `members` for you. (`--vcs none`
stops it from creating a git repository inside each crate.)

In `labstats-cli/Cargo.toml`:

```toml
[[bin]]
name = "labstats"
path = "src/main.rs"

[dependencies]
labstats-core = { path = "../labstats-core" }
```

Then:

1. Move the types and functions into `labstats-core/src/lib.rs`. Mark them
   `pub`, and the struct fields too, otherwise the CLI can't see them.
2. In the CLI, `use labstats_core::{parse, parse_line, report};` (dashes in
   crate names become underscores in code).
3. Fix the ugly usage message: rename `main` to `run`, and write a new `main`
   that calls `run()`, prints the error with `eprintln!("{e}")` and exits with
   `std::process::exit(1)`.
4. Add tests at the bottom of `lib.rs`:

   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn parses_a_valid_line() {
           let m = parse_line("A1, 0.412").unwrap();
           assert_eq!(m.sample, "A1");
           assert_eq!(m.value, 0.412);
       }
   }
   ```

5. From the workspace root:

   ```sh
   cargo test --workspace
   cargo run -p labstats-cli -- summary data/measurements.csv
   cargo doc --open
   ```

Note that there is a single `Cargo.lock` and a single `target/` folder for the
whole workspace.

Checkpoint: `checkpoints/step6`.

---

## Bonus ideas

If you finish early, pick one:

- Add the standard deviation to `Summary`.
- Sort the report by mean instead of by name (`sort_by` with
  `f64::total_cmp`).
- Replace the hand-written argument parsing with the
  [`clap`](https://docs.rs/clap) crate (`cargo add clap --features derive`).
- Replace the hand-written parsing with the [`csv`](https://docs.rs/csv) and
  [`serde`](https://serde.rs) crates.
- Define your own error type as an `enum` instead of `String`, and implement
  `std::fmt::Display` for it.
- Add a third crate to the workspace, for example a tool that generates
  random measurement files with `rand`.
