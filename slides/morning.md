---
marp: true
paginate: true
title: Rust workshop, morning
style: |
  section { font-size: 28px; }
  code { font-size: 0.9em; }
  table { font-size: 0.8em; }
---

# Rust workshop

### Morning: the ecosystem and the language

9:00 to 12:00

---

## Today

**Morning**
- What Rust is and why people use it
- Installing it, cargo, dependencies
- Hello world
- The language: variables, functions, control flow, structs, enums, errors

**Afternoon**
- A mini project: read a data file, compute stats, save results
- Memory: ownership and borrowing
- Cargo workspaces

---

## What is Rust?

- A compiled, statically typed language, first stable release in 2015
- As fast as C and C++: no garbage collector, no runtime
- **Memory safe** without a garbage collector: the compiler checks it
- No data races in multithreaded code, also checked by the compiler
- Excellent tooling, all official: build, packages, tests, docs, formatter, linter

> "If it compiles, it usually works" is an exaggeration, but not a big one.

---

## Who uses it?

- Linux kernel, Windows kernel, Android, Chromium, Firefox
- Cloud infrastructure: AWS (Firecracker), Cloudflare, Discord
- **Python tooling**: `uv`, `ruff`, `polars`, `pydantic-core`, `cryptography`
  are all written in Rust
- Science: fast Python extensions with PyO3, numerics with `ndarray`,
  `nalgebra`, data with `polars`

Recurring story: "we rewrote the slow part of our Python in Rust and it's now
50x faster, and it doesn't segfault."

---

## Why not just C++?

| | C++ | Rust |
|---|---|---|
| Speed | ✔ | ✔ |
| Use-after-free, dangling pointers | possible | compile error |
| Data races | possible | compile error |
| Null pointers | yes | no null, `Option<T>` instead |
| Build system | CMake, Meson, Bazel... | `cargo` |
| Packages | vcpkg, conan, copy the source | `cargo add` |
| Tests, docs, formatting | pick a tool | built in |

The price: a steeper start. The compiler is strict, and will refuse code
until it can prove it's safe.

---

## Why not just Python?

- Python is great for gluing things together and exploring
- Rust is for the parts where you need:
  - **speed** (10x to 100x on loops is common)
  - **reliability**: types and errors checked before running
  - **a single binary** to hand to someone, no environment to set up
- They work well together: write the hot loop in Rust, call it from Python

---

## The ecosystem

| Tool | What it does | Python equivalent |
|---|---|---|
| `rustup` | installs and updates the toolchain | `pyenv` |
| `rustc` | the compiler | `python` |
| `cargo` | build, run, test, dependencies | `pip` + `pytest` + `venv` + ... |
| crates.io | the package registry | PyPI |
| docs.rs | documentation of every crate | readthedocs |
| `rustfmt` | formatter (`cargo fmt`) | `black` |
| `clippy` | linter (`cargo clippy`) | `ruff` |
| rust-analyzer | editor support | Pylance |

---

## Installation check

```sh
rustc --version
cargo --version
rustup update
```

If you didn't install it yet:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Windows: <https://rustup.rs>

Editor: VS Code + **rust-analyzer** extension

---

## Hello world, the hard way

`helloworld.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

```sh
rustc helloworld.rs
./helloworld
```

- `fn main()` is the entry point
- `println!` has a `!`: it's a **macro**, not a function
- `rustc` produces a native executable, nothing else needed to run it

---

## Hello world, the normal way

```sh
cargo new hello
cd hello
cargo run
```

```
hello/
├── Cargo.toml      # project manifest: name, version, dependencies
├── .gitignore
└── src/
    └── main.rs     # same hello world as before
```

Nobody calls `rustc` directly. Cargo does it for you.

---

## Cargo.toml

```toml
[package]
name = "hello"
version = "0.1.0"
edition = "2024"

[dependencies]
```

- `edition`: language version. Editions can change syntax, crates of
  different editions still work together
- `[dependencies]`: what you need from crates.io

---

## Adding a dependency

```sh
cargo new dice
cd dice
cargo add rand
```

```toml
[dependencies]
rand = "0.10.3"
```

`src/main.rs`:

```rust
fn main() {
    let rolls: Vec<u32> = (0..10).map(|_| rand::random_range(1..=6)).collect();
    println!("rolls: {rolls:?}");

    let total: u32 = rolls.iter().sum();
    println!("total: {total}");
}
```

---

## Versions and Cargo.lock

- `rand = "0.10.3"` means "0.10.3 or any compatible newer version"
  (semantic versioning: `>=0.10.3, <0.11.0`)
- `Cargo.lock` records the **exact** versions that were used
- Commit `Cargo.lock` for programs, so everyone builds the same thing
- `cargo update` to move to newer compatible versions
- `cargo tree` to see all the dependencies

---

## Cargo commands

```sh
cargo run               # build + run
cargo run -- arg1 arg2  # pass arguments to your program
cargo check             # only type check: fast, use it all the time
cargo build --release   # optimized build, can be 10x+ faster than debug
cargo test              # run the tests
cargo fmt               # format the code
cargo clippy            # lint: catches mistakes and unidiomatic code
cargo doc --open        # docs for your code *and* all dependencies, offline
```

Debug builds go to `target/debug/`, release builds to `target/release/`.

---

<!-- _class: lead -->

# The language

Exercises: `morning/exercises`

```sh
cd morning/exercises
cargo run --bin ex01_variables
```

---

## Variables

```rust
let x = 5;            // immutable by default
x = 6;                // error: cannot assign twice to immutable variable

let mut y = 5;        // opt in to mutability
y += 1;

let z: f64 = 2.5;     // type annotation (usually inferred)

const MAX_SAMPLES: u32 = 100;   // compile-time constant, type required
```

- Immutable by default: when you see `mut`, you know it will change
- Python: everything is mutable. C++: `const` is opt in. Rust: the reverse.

---

## Shadowing

```rust
let input = "21.5";                  // &str
let input: f64 = input.parse().unwrap();  // new variable, same name, f64
```

- `let` again creates a **new** variable that hides the old one
- Handy for converting a value without inventing `input_str`, `input_num`...
- Different from `mut`: the type can change

---

## Basic types

| Rust | Meaning |
|---|---|
| `i8`, `i16`, `i32`, `i64`, `i128` | signed integers (`i32` default) |
| `u8` ... `u128`, `usize` | unsigned integers, `usize` for indices and sizes |
| `f32`, `f64` | floats (`f64` default) |
| `bool` | `true`, `false` |
| `char` | one Unicode character, `'a'`, `'é'`, `'🦀'` |
| `(i32, f64)` | tuple |
| `[i32; 3]` | fixed-size array |

**No implicit conversions**, not even `i32` to `f64`:

```rust
let ratio = 7 / 2.0;          // error
let ratio = 7 as f64 / 2.0;   // 3.5
```

---

## Integer overflow

```rust
let small: u8 = 300;   // compile error: literal out of range for `u8`
```

- In debug builds, overflow at runtime **panics** (crashes cleanly)
- In release builds, it wraps around
- If you want a specific behaviour, ask for it:
  `x.wrapping_add(1)`, `x.checked_add(1)`, `x.saturating_add(1)`

Python has unbounded integers. C++ has undefined behaviour for signed
overflow. Rust makes you choose.

---

## Exercise 1: variables

```sh
cargo run --bin ex01_variables
```

The file does **not** compile. Fix it.

- Read the error messages: they tell you what's wrong and often how to fix it
- Errors come in rounds: fix some, others appear
- `rustc --explain E0384` for the full explanation of an error code

---

## Functions

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn greet(name: &str) {
    println!("Hello, {name}!");
}
```

- Parameter types are always required, return type after `->`
- The **last expression** without `;` is the return value
- `return x;` exists for early returns
- No return type means it returns `()`, the empty tuple ("nothing")

---

## Expressions everywhere

Almost everything is an expression that produces a value:

```rust
let sign = if x >= 0 { "positive" } else { "negative" };

let y = {
    let a = 3;
    a * 2           // no semicolon: this is the block's value
};                  // y == 6
```

Adding a `;` turns an expression into a statement, whose value is `()`:

```rust
fn double(x: i32) -> i32 {
    x * 2;          // error: expected `i32`, found `()`
}
```

---

## Printing and formatting

```rust
let name = "A1";
let value = 0.41234;

println!("{name} = {value}");        // A1 = 0.41234
println!("{name} = {value:.2}");     // A1 = 0.41
println!("{:>8}|", name);            // "      A1|"
println!("{:?}", vec![1, 2, 3]);     // [1, 2, 3]   debug format
println!("{:#?}", some_struct);      // pretty debug format

let s = format!("{name}: {value:.3}");   // same, but returns a String
eprintln!("to stderr");
```

Same mini language as Python f-strings.

---

## Exercise 2: functions

```sh
cargo run --bin ex02_functions
```

Replace each `todo!()` with a real implementation until the program prints
`ex02 done!`.

`todo!()` is a macro that compiles to "crash here with 'not yet implemented'".
Very useful to sketch a program.

---

## Control flow

```rust
if temperature > 30.0 {
    println!("hot");
} else if temperature > 15.0 {
    println!("fine");
} else {
    println!("cold");
}

while n != 1 {
    n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
}

for i in 0..10 { }          // 0 to 9
for i in 0..=10 { }         // 0 to 10
for x in &values { }        // each element of a list
for (i, x) in values.iter().enumerate() { }

loop {                      // forever, until `break`
    break;
}
```

No parentheses around conditions, braces always required.

---

## `match`

```rust
match score {
    90..=100 => 'A',
    75..=89 => 'B',
    50..=74 => 'C',
    _ => 'F',               // `_` matches anything else
}
```

- Like `switch`, but it's an expression, and works with ranges,
  tuples, structs, enums...
- **Must be exhaustive**: forget a case and it doesn't compile

---

## No exercise for this one

Loops and `if` work the way you expect. There is an exercise anyway,
`ex03_control_flow` (Collatz steps, letter grades, counting), for whoever
finishes the others early, or for later.

---

## Structs

```rust
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn square(size: f64) -> Rectangle {          // no self: "static" method
        Rectangle { width: size, height: size }
    }

    fn area(&self) -> f64 {                     // &self: borrows the struct
        self.width * self.height
    }
}

let r = Rectangle { width: 3.0, height: 4.0 };
let s = Rectangle::square(2.0);
println!("{} {:?}", r.area(), s);
```

Data (`struct`) and behaviour (`impl`) are separate. No inheritance.

---

## Enums

Much more than C enums: each variant can carry data.

```rust
enum Shape {
    Circle { radius: f64 },
    Rect(Rectangle),
    Triangle { base: f64, height: f64 },
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle { radius } => 3.14159 * radius * radius,
        Shape::Rect(r) => r.area(),
        Shape::Triangle { base, height } => base * height / 2.0,
    }
}
```

Add a new variant and the compiler shows you every `match` to update.

---

## `#[derive]`

```rust
#[derive(Debug, Clone, PartialEq)]
struct Measurement { sample: String, value: f64 }
```

Asks the compiler to write common code for you:

| Derive | Gives you |
|---|---|
| `Debug` | printing with `{:?}` |
| `Clone` | `.clone()` |
| `PartialEq` | `==` and `assert_eq!` |
| `Default` | `Measurement::default()` |

These are **traits**, Rust's version of interfaces.

---

## Exercise 4: structs and enums

```sh
cargo run --bin ex04_structs_enums
```

---

## No null: `Option`

```rust
enum Option<T> {
    Some(T),
    None,
}
```

```rust
fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

match safe_divide(1.0, x) {
    Some(r) => println!("result: {r}"),
    None => println!("division by zero"),
}
```

The type tells you a value may be missing, and you can't use it without
handling the `None` case. No more `NoneType has no attribute...` or segfaults.

---

## No exceptions: `Result`

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

```rust
fn parse_celsius(text: &str) -> Result<f64, String> {
    let t: f64 = match text.trim().parse() {
        Ok(t) => t,
        Err(e) => return Err(format!("not a number: {e}")),
    };
    if t < -273.15 {
        return Err(format!("{t} is below absolute zero"));
    }
    Ok(t)
}
```

Errors are ordinary values. A function that can fail says so in its type.

---

## The `?` operator

```rust
fn parse_kelvin(text: &str) -> Result<f64, String> {
    let celsius = parse_celsius(text)?;   // on Err: return the error now
    Ok(celsius + 273.15)
}
```

`?` means "if this is an error, return it to my caller, otherwise give me the
value". Like an exception propagating, but visible in the code.

Quick and dirty, for prototypes:

```rust
let t = parse_celsius("21.5").unwrap();        // panics on Err
let t = parse_celsius("21.5").expect("bad");   // panics with a message
```

---

## Exercise 5, after lunch

```sh
cargo run --bin ex05_option_result
```

The project this afternoon starts with it: reading a file is the first thing
that can fail.

---

## Recap

- `cargo` does everything: new, run, test, add, fmt, clippy, doc
- Variables are immutable unless `mut`, types don't convert implicitly
- Blocks, `if` and `match` are expressions
- `struct` + `impl` for data and methods, `enum` + `match` for alternatives
- `Option` instead of null, `Result` instead of exceptions, `?` to propagate

**This afternoon**: a real program, and the thing that makes Rust Rust:
ownership. Why did `ex01` need `.parse()`, and what is that `&` in `&str`?

---

# Lunch 🦀

Back at 14:00
