
### Dev Setting

- `rustup`: a command line tool for managing  Rust versions. like conda?
  - update rust version: `rustup update`
- `rustc`: your rust compiler
  - check your rustc version: `rustc --version`


### Cargo

- A rust build system and package manager, like pip
  - Download the libraries depends on and build them
  - Build code: `cargo build`
    - default is build for debugging, the binary file would under `./target/debug/xxx`
    - build for release: `cargo build --release`
      - the binary would under `./target/release/xxx`
  - Create a project: `cargo new project_dir`
    - It would create a `Cargo.toml` under the project dir
  - Build and run
    - If you want to build debug mode and run directly: `cargo run`
    - If you want to build release mode and run directly: `cargo run --release`
  - Add an external crate to the dependency of a project: `cargo add xxx`
