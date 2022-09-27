# How to contribute

If you want to add a cool module just implement it, lint it with [clippy](https://github.com/rust-lang-nursery/rust-clippy)
and make a pull request with a screenshot. I will probably accept it.

## Building for the web

The `trunk` cli is required to build the web version. To add this, run `cargo install trunk`.

## Code details

- Make sure your runs on wasm32 as well as on other platforms. For instance, please don't use `println!()` but instead
  prefer `io::dprint()`, `io::newline()`, or `io::print()`.
