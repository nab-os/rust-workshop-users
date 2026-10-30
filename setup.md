# Rust workshop: setup before the day

Please do this before the workshop. It takes about 15 minutes, and it means we
can spend the morning writing code instead of fighting installers and wifi.

## 1. Install Rust with rustup

rustup installs the compiler (`rustc`), the build tool (`cargo`) and the
standard library, and keeps them up to date. It installs into your home
directory, so you don't need admin rights on Linux or macOS.

**Linux / macOS**

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Choose the default installation, then open a new terminal.

You also need a C linker:

- Debian/Ubuntu: `sudo apt install build-essential`
- Fedora: `sudo dnf install gcc`
- macOS: `xcode-select --install`

**Windows**

Download and run `rustup-init.exe` from <https://rustup.rs>. It will offer to
install the Visual Studio C++ Build Tools: say yes, they provide the linker.
(If you'd rather work inside WSL, follow the Linux instructions there instead.)

## 2. Check that it works

```sh
rustc --version
cargo --version
```

Both should print a version number, 1.85 or newer.

## 3. Set up an editor

Any editor works, but you'll have a much better time with **rust-analyzer**,
which gives you inline type hints, errors as you type and autocompletion.

- VS Code: install the **rust-analyzer** extension (the one published by
  "The Rust Programming Language"). Don't install the old "Rust" extension.
- RustRover (JetBrains): free for non-commercial use, works out of the box.
- Vim, Neovim, Emacs, Helix, Zed: rust-analyzer is available through your
  usual LSP setup.

## 4. Smoke test (also downloads the crate we use in the morning)

```sh
cargo new hello
cd hello
cargo add rand
cargo run
```

You should see `Hello, world!`. If it fails, bring the error message on the
day or send it to me beforehand and we'll sort it out.

## Optional reading

None of this is required, but if you're curious:

- [The Rust Book](https://doc.rust-lang.org/book/), chapters 1 to 3
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
