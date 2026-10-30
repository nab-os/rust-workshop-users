# Python / C++ / Rust cheat sheet

| | Python | C++ | Rust |
|---|---|---|---|
| Run a program | `python main.py` | `g++ main.cpp && ./a.out` | `cargo run` |
| Package manager | `pip`, `uv` | CMake + vcpkg/conan | `cargo` (built in) |
| Package registry | PyPI | (none official) | crates.io |
| Add a dependency | `pip install numpy` | ... | `cargo add rand` |
| Variable | `x = 5` | `int x = 5;` | `let x = 5;` (immutable) |
| Mutable variable | `x = 5` | `int x = 5;` | `let mut x = 5;` |
| Constant | `MAX = 10` (convention) | `constexpr int MAX = 10;` | `const MAX: i32 = 10;` |
| Function | `def add(a, b): return a + b` | `int add(int a, int b) { return a + b; }` | `fn add(a: i32, b: i32) -> i32 { a + b }` |
| String | `"hi"` | `std::string`, `const char*` | `String` (owned), `&str` (borrowed) |
| Format | `f"{x} and {y:.2f}"` | `std::format("{} and {:.2f}", x, y)` | `format!("{x} and {y:.2}")` |
| Print | `print(x)` | `std::cout << x << '\n';` | `println!("{x}");` |
| Debug print | `print(repr(x))` | ... | `println!("{x:?}");` |
| List | `[1, 2, 3]` | `std::vector<int>{1, 2, 3}` | `vec![1, 2, 3]` (`Vec<i32>`) |
| Dict | `{"a": 1}` | `std::unordered_map` | `HashMap` / `BTreeMap` |
| Loop over items | `for x in xs:` | `for (auto& x : xs)` | `for x in &xs { }` |
| Range | `range(0, 10)` | ... | `0..10`, `0..=9` |
| With index | `for i, x in enumerate(xs):` | ... | `for (i, x) in xs.iter().enumerate()` |
| Nothing | `None` | `nullptr`, `std::optional` | `Option<T>`: `Some(x)` / `None` |
| Errors | exceptions, `try/except` | exceptions or error codes | `Result<T, E>`: `Ok(x)` / `Err(e)`, `?` to propagate |
| Switch | `match` (3.10+) | `switch` | `match` (must cover every case) |
| Class | `class Point:` | `struct Point { };` | `struct Point { x: f64, y: f64 }` + `impl Point { }` |
| Method on self | `def norm(self):` | `double norm() const` | `fn norm(&self) -> f64` |
| Interfaces | duck typing, ABCs | virtual classes, concepts | traits |
| Memory | garbage collector | manual / RAII / smart pointers | ownership + RAII, checked at compile time |
| `b = a` (big object) | two names, one object | copy | **move**: `a` can't be used anymore |
| Explicit copy | `copy.deepcopy(a)` | `auto b = a;` | `let b = a.clone();` |
| Reference | (everything is) | `const T&`, `T&` | `&T`, `&mut T` |
| Heap pointer | (everything is) | `std::unique_ptr<T>` | `Box<T>` |
| Shared pointer | (everything is) | `std::shared_ptr<T>` | `Rc<T>`, `Arc<T>` (threads) |
| Integer division | `7 // 2` | `7 / 2` | `7 / 2` |
| Mixed int/float | `7 / 2.0` works | `7 / 2.0` works | `7 as f64 / 2.0` (no implicit conversion) |
| Tests | `pytest` | GoogleTest, Catch2 | `#[test]` + `cargo test` (built in) |
| Formatter | `black`, `ruff format` | `clang-format` | `cargo fmt` |
| Linter | `ruff`, `pylint` | `clang-tidy` | `cargo clippy` |
| Docs | Sphinx | Doxygen | `cargo doc --open`, docs.rs |

## Cargo commands you'll use today

```sh
cargo new name          # new binary project (git repo included)
cargo new --lib name    # new library project
cargo run               # build and run
cargo run -- args       # pass arguments to your program
cargo check             # type check only, much faster than build
cargo build --release   # optimized build in target/release
cargo test              # run the tests
cargo add crate         # add a dependency to Cargo.toml
cargo fmt               # format the code
cargo clippy            # lint
cargo doc --open        # documentation for your code and its dependencies
```

## Reading a compiler error

```
error[E0382]: borrow of moved value: `sample`
 --> src/bin/own01_move.rs:9:25
  |
6 |     let sample = String::from("A1");
  |         ------ move occurs because `sample` has type `String`, which does not implement the `Copy` trait
7 |     let label = sample;
  |                 ------ value moved here
8 |
9 |     println!("sample = {sample}");
  |                         ^^^^^^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
7 |     let label = sample.clone();
  |                       ++++++++
```

1. The first line says what is wrong.
2. The arrows point to *every* line involved, not just the last one.
3. The `help:` at the end often contains the fix.
4. `rustc --explain E0382` gives a full explanation with examples.
