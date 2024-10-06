# Installation on Linux

- $ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
  ![Image](/Rust-The-book/00_Installation_Setup/Images/1.png)

- Select 1
  ![Image2](/Rust-The-book/00_Installation_Setup/Images/2.png)

### Rust is installed now. Great !

# Hello World

- mkdir hello_world
- cd hello_world
- create a file `main.rs`

```Rust
fn main() {
    println!("Hello World")
}

```

- to execute the file
- `rustc main.rs` to compile
- ./main

---

# Cargo

- `cargo --version`

- Creating a Project with Cargo

```rust
$ cargo new hello_cargo
$ cd hello_cargo
```

- Building and Running a Cargo Project
  - `cd hello cargo`

```rust
$ cargo build
$ cargo run
```

```rust
$ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!

```
