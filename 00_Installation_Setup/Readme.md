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