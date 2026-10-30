---
marp: true
paginate: true
title: Rust workshop, afternoon
style: |
  section { font-size: 28px; }
  code { font-size: 0.9em; }
  table { font-size: 0.8em; }
---

# Rust workshop

### Afternoon: a mini project

14:00 to 17:00

---

## Quick recap

```rust
let mut total = 0.0;                         // mut to change it
fn mean(values: &[f64]) -> f64 { ... }       // last expression is returned
struct Measurement { sample: String, value: f64 }
enum Shape { Circle { radius: f64 }, ... }   // variants carry data
match x { ... }                              // exhaustive
Option<T>  // Some(x) or None
Result<T, E>  // Ok(x) or Err(e), propagate with ?
```

---

## Warm-up: Option and Result

```sh
cd morning/exercises
cargo run --bin ex05_option_result
```

Division that can fail, a search that may find nothing, parsing a
temperature. 15 minutes, then we start the project.

---

## The project: `labstats`

A command line tool for a file of lab measurements.

```console
$ labstats summary data/measurements.csv
warning: line 12: bad value `oops`: invalid float literal
warning: line 17: empty sample name
sample  count   mean    min     max
A1      5       0.407   0.398   0.420
B2      4       1.284   1.250   1.310
...

$ labstats add data/measurements.csv C3 2.001
```

Instructions: `afternoon/project.md`

---

## The plan

| Step | What | Rust topic |
|---|---|---|
| 1 | Read a text file | `std::fs`, `Result`, `?` |
| 2 | Parse lines into a list | `struct`, `Vec`, string methods |
| 3 | Ownership and borrowing | memory management |
| 4 | Group and summarize | `HashMap`, `BTreeMap`, iterators |
| 5 | Save a report, append data | writing files |
| 6 | Library + program (demo) | cargo workspaces, tests |

**Lost? Copy `checkpoints/stepN` and carry on.**

---

## Step 1: reading a file

```rust
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args().nth(1).ok_or("usage: labstats <file>")?;
    let text = fs::read_to_string(&path)?;

    for (i, line) in text.lines().enumerate() {
        println!("{:>3} | {line}", i + 1);
    }
    Ok(())
}
```

```sh
cargo run -- data/measurements.csv
```

`Box<dyn Error>`: "some error, any kind". Fine for applications.

---

## Step 2: parsing

```rust
#[derive(Debug, Clone, PartialEq)]
struct Measurement {
    sample: String,
    value: f64,
}

fn parse_line(line: &str) -> Result<Measurement, String> { todo!() }

fn parse(text: &str) -> (Vec<Measurement>, Vec<String>) { todo!() }
```

- `split_once(',')`, `trim()`, `parse::<f64>()`
- `ok_or(...)`: `Option` to `Result`
- `map_err(...)`: change the error type

Why `String` in the struct but `&str` in the function?

---

<!-- _class: lead -->

# Step 3: memory

How Rust manages memory without a garbage collector

---

## Stack and heap

```rust
let n: i32 = 42;
let s: String = String::from("control");
```

```
      stack                       heap
  ┌────────────┐
n │     42     │
  ├────────────┤             ┌─┬─┬─┬─┬─┬─┬─┐
s │ ptr ───────┼───────────▶ │c│o│n│t│r│o│l│
  │ len = 7    │             └─┴─┴─┴─┴─┴─┴─┘
  │ cap = 7    │
  └────────────┘
```

`n` has a fixed size, it lives on the stack and disappears with the
function. The text in `s` can grow, so it lives on the heap, and somebody
has to give that memory back at some point.

<!-- Draw this on the board rather than showing the slide, it sticks better. -->

---

## Three answers to "who frees it?"

In C, you do, by hand. Forget and you leak, do it twice and you crash.

In Python (and Java, Go...), a garbage collector does it while the program
runs. Convenient, costs some speed and some predictability.

In C++, destructors do it when a variable goes out of scope. That works
well, as long as nobody keeps a pointer to something that was destroyed.
Nothing stops you from doing that.

Rust takes the C++ idea, scope-based, no runtime cost, and adds a compiler
check that nobody is still pointing at the memory when it's freed. That
check is called ownership, and it's what the rest of this hour is about.

---

## The ownership rules

1. Every value has one owner, a variable.
2. When the owner goes out of scope, the value is freed.
3. Assigning or passing the value moves it to a new owner. The old one
   can't be used anymore.

```rust
{
    let s = String::from("A1");   // s owns the string
    let t = s;                    // now t does
    println!("{s}");              // error: s was moved
}                                 // t goes out of scope, string freed once
```

Rule 3 is the one that surprises everyone. It's what makes rule 2 safe:
there is exactly one variable responsible for freeing, so it happens exactly
once.

---

## Move vs copy

```rust
let s = String::from("A1");
let t = s;              // move: copies ptr/len/cap, s is now invalid
let u = t.clone();      // explicit deep copy, new heap allocation

let a = 5;
let b = a;              // i32 is `Copy`: a is still valid
```

Plain numbers, `bool`, `char` and tuples of those are copied, since they're
just a few bytes on the stack. Anything with heap memory behind it moves,
and copying it is always spelled out with `.clone()`.

For the C++ people: `auto t = s;` copies in C++ and moves in Rust, and
`std::move` is the default rather than something you ask for. Using a moved
variable is a compile error, not undefined behaviour.

---

## Moving into functions

```rust
fn describe(name: String) -> String {
    format!("sample {name}")
}                                   // name dropped here

let name = String::from("B2");
describe(name);
describe(name);     // error: name was moved into the first call
```

Taking ownership is rarely what you want for a function that just *reads*.

---

## Borrowing

```rust
fn describe(name: &str) -> String {
    format!("sample {name}")
}

let name = String::from("B2");
describe(&name);    // lend it
describe(&name);    // lend it again
                    // name is still owned here
```

- `&T`: a shared reference, read only
- `&mut T`: a mutable reference, can modify
- References never own: nothing is freed when they go away

---

## The borrowing rule

At any time, either any number of read-only references `&T`, or a single
`&mut T`. Never both. And a reference can't outlive what it points to.

```rust
let mut readings = vec![0.41, 0.39];
let first = &readings[0];
readings.push(0.40);          // error: readings is borrowed
println!("{first}");
```

`push` may move the whole vector to a bigger block of memory, and `first`
would then point into the old, freed block. In C++ this compiles and runs,
and it's a bug you find only when the vector happens to grow.

<!-- This is the moment to tell your own vector-invalidation story if you have one. -->

---

## Slices: views into data

```rust
let text: String = String::from("A1,0.412");
let sample: &str = &text[..2];          // "A1", no copy

let values: Vec<f64> = vec![0.4, 1.3, 2.0];
let firsts: &[f64] = &values[..2];      // [0.4, 1.3], no copy
```

| Owns the data | Borrowed view |
|---|---|
| `String` | `&str` |
| `Vec<T>` | `&[T]` |

Rule of thumb for function parameters: take `&str` and `&[T]`, they accept
both owned values (via `&`) and slices of them.

---

## Back to the question

```rust
struct Measurement { sample: String, value: f64 }
fn parse_line(line: &str) -> Result<Measurement, String>
```

`parse_line` only reads the line, so it borrows it: `&str`.

The `Measurement` has to own its sample name, because it will still be
around once the file contents have been dropped. That's why step 2 needed
`.to_string()`.

---

## Exercises

```sh
cd ownership-exercises
cargo run --bin own01_move
```

| | |
|---|---|
| `own01_move` | moves, `.clone()`, `Copy` |
| `own02_borrow` | borrowing instead of taking ownership |
| `own03_mut_borrow` | `&` and `&mut` together |
| `own04_slices` | `&[T]`, `&mut [T]`, `&str` |
| `own05_lifetimes` | bonus |

---

## Beyond single ownership

When one owner isn't enough:

| Type | Use |
|---|---|
| `Box<T>` | a value on the heap, one owner (`std::unique_ptr`) |
| `Rc<T>` | shared ownership, reference counted (`std::shared_ptr`) |
| `Arc<T>` | same, safe across threads |
| `RefCell<T>`, `Mutex<T>` | mutation through a shared reference, checked at runtime |

You won't need any of them today. `Vec`, `String` and references cover most
programs.

---

# Break ☕

Back at 15:45

---

## Step 4: group and summarize

```rust
use std::collections::HashMap;

fn group_by_sample(measurements: &[Measurement]) -> HashMap<&str, Vec<f64>> {
    let mut groups = HashMap::new();
    for m in measurements {
        groups.entry(m.sample.as_str()).or_default().push(m.value);
    }
    groups
}
```

- The keys are `&str` **borrowed** from `measurements`: no copies
- The compiler tracks that the map can't outlive `measurements`
- Try `drop(measurements);` before using the map

---

## Iterators

```rust
let total: f64 = values.iter().sum();
let mean = total / values.len() as f64;
let big: Vec<f64> = values.iter().copied().filter(|&v| v > 1.0).collect();
let names: Vec<&str> = measurements.iter().map(|m| m.sample.as_str()).collect();
```

- Lazy, chained, and compiled down to the same code as a hand-written loop
- `|x| ...` is a closure (Python's `lambda x: ...`)
- `collect()` builds a collection, the type annotation says which

---

## HashMap vs BTreeMap

Run step 4 twice. The order changes!

| | `HashMap` | `BTreeMap` |
|---|---|---|
| Order | random (per run) | sorted by key |
| Lookup | O(1) | O(log n) |
| Python | `dict` (insertion order) | `sorted(dict.items())` |

Switch the type: one word changes, nothing else.

---

## Step 5: writing files

```rust
use std::fs::{self, OpenOptions};
use std::io::Write;

fs::write("report.tsv", &table)?;              // whole file

let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open(path)?;
writeln!(file, "{},{}", m.sample, m.value)?;   // append a line
```

The file is closed automatically when `file` goes out of scope (RAII again).

---

## Step 5: subcommands

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

`match` on strings, with a guard (`if ...`) on each arm. Good enough here.
For a real tool, use the `clap` crate.

---

## Step 6: workspaces

```
labstats-ws/
├── Cargo.toml            [workspace] members = [...]
├── Cargo.lock            one for everyone
├── target/               one for everyone
├── data/
├── labstats-core/        library: parsing, stats, tests
│   ├── Cargo.toml
│   └── src/lib.rs
└── labstats-cli/         binary: arguments and files
    ├── Cargo.toml        labstats-core = { path = "../labstats-core" }
    └── src/main.rs
```

- Split a project into crates that are built and tested together
- Others can reuse `labstats-core` without the CLI

---

## Visibility and tests

```rust
pub struct Measurement {
    pub sample: String,     // fields are private by default too
    pub value: f64,
}

pub fn parse_line(line: &str) -> Result<Measurement, String> { ... }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_bad_lines() {
        assert!(parse_line("A1,abc").is_err());
    }
}
```

```sh
cargo test --workspace
```

---

## What we covered today

- rustup, cargo, crates.io, Cargo.toml and Cargo.lock
- Variables, types, functions, control flow, structs, enums, `match`
- `Option` and `Result` instead of null and exceptions
- Reading and writing files
- `Vec`, `HashMap`, `BTreeMap`, iterators
- Ownership, borrowing, slices
- Workspaces, libraries, unit tests

---

## Where to go next

- **The Rust Book**: doc.rust-lang.org/book
- **Rustlings**: small exercises like this morning's
- **Rust by Example**, **Exercism** Rust track
- **PyO3** + **maturin**: Python modules written in Rust
- Crates worth knowing: `clap`, `serde`, `anyhow`, `rayon`, `ndarray`,
  `polars`, `plotters`

---

# Thank you 🦀

Questions?
