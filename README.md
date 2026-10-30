# Rust workshop

A one-day introduction to Rust, for people who know Python and maybe some C++.
No Rust experience needed.

**Before the day, follow [`setup.md`](setup.md)** to install Rust and an editor.
It takes about 15 minutes.

## Programme

**9:00 to 12:00: the ecosystem and the language**

Installation, cargo and dependencies, hello world, then variables, functions,
control flow, structs, enums, `Option` and `Result`, with a short exercise
after each topic.

**14:00 to 17:00: mini project `labstats`**

A small command line tool that reads a file of lab measurements, computes
statistics per sample, saves a report and records new measurements. Along
the way: reading and writing files, lists and maps, ownership and borrowing,
and splitting a project into a cargo workspace.

## What's in here

| Path | What |
|---|---|
| `setup.md` | installation instructions |
| `cheatsheet.md` | Python / C++ / Rust side by side, cargo commands, reading compiler errors |
| `slides/` | the slides for both sessions |
| `morning/exercises/` | morning exercises `ex01` to `ex05` |
| `afternoon/project.md` | step by step instructions for the project |
| `afternoon/data/` | the input file for the project |
| `afternoon/ownership-exercises/` | memory exercises `own01` to `own05` |
| `afternoon/checkpoints/` | a working version of the project after each step |

## Getting the material

```sh
git clone <url of this repository>
cd rust-workshop-participants
```

Run `git pull` on the morning of the workshop to get any last changes.

## Morning exercises

```sh
cd morning/exercises
cargo run --bin ex01_variables
```

Each exercise is its own small program in `src/bin/`. Run them one at a time
with `--bin`, so a broken one doesn't stop the others from building.

- `ex01_variables` doesn't compile at first: your job is to fix it by reading
  the compiler errors. They come in rounds, so after you fix the first ones
  a few more appear. That's normal.
- `ex02` to `ex05` compile, but crash with `not yet implemented` until you
  replace every `todo!()`. Warnings about unused variables are expected
  until then.
- An exercise is finished when it prints `exNN done!`.

## Afternoon project

Follow [`afternoon/project.md`](afternoon/project.md).

**If you get lost, don't worry:** copy the folder for the previous step from
`afternoon/checkpoints/` and continue from there. Nobody is expected to
finish every step on their own.

```sh
cp -r afternoon/checkpoints/step2 my-labstats
cd my-labstats
cargo run -- data/measurements.csv
```

Step 3 uses the exercises in `afternoon/ownership-exercises`, run the same way
as the morning ones (`cargo run --bin own01_move`).

## Viewing the slides

They're plain markdown and read fine on GitHub or in any editor. To present
or print them, use the "Marp for VS Code" extension, or:

```sh
npx @marp-team/marp-cli slides/morning.md --html
```

## Common problems

| Symptom | Fix |
|---|---|
| `linker 'cc' not found` | Install `build-essential` (Debian/Ubuntu), `gcc` (Fedora), or run `xcode-select --install` (macOS). |
| `link.exe not found` on Windows | The Visual Studio Build Tools are missing. Re-run `rustup-init.exe` and accept them, or use WSL. |
| rust-analyzer shows nothing | Open the folder that contains `Cargo.toml` (for example `morning/exercises`), not a parent folder. |
| `could not find Cargo.toml` | You're in the wrong directory, `cd` into the project first. |
| `No such file or directory` when reading the CSV | Paths are relative to where you run `cargo run`. Run it from the project folder: `cargo run -- data/measurements.csv`. |
| My arguments are ignored | Arguments for your program go after `--`: `cargo run -- summary data/measurements.csv`. |
| Error mentioning `edition2024` | Your Rust is too old. Run `rustup update`. |
| Downloads from crates.io fail | Probably the proxy or the wifi. Set `HTTPS_PROXY` if your network needs one, or ask for help. |
| I don't understand an error | Read the `help:` lines at the end, and try `rustc --explain E0382` with the error code. See `cheatsheet.md`. |

## After the workshop

- [The Rust Book](https://doc.rust-lang.org/book/), the reference introduction
- [Rustlings](https://rustlings.rust-lang.org/), small exercises in the same
  style as this morning's
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Exercism Rust track](https://exercism.org/tracks/rust), exercises with mentoring
- [PyO3](https://pyo3.rs) and [maturin](https://www.maturin.rs), to write
  Python modules in Rust
- Crates worth knowing: `clap` (command line arguments), `serde` (serialization),
  `anyhow` (error handling), `rayon` (easy parallelism), `ndarray`, `polars`,
  `plotters`
