# Learning Rust
The goal of this repository is to document my Rust learning process.

## Installing Rust
To install it locally, simply run the following command in the terminal:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

And also add it to your PATH.
```bash
source "$HOME/.cargo/env"
```

You can check the current version for Rust installed on your machine:
```bash
rustup --version
```

## Creating a Rust "Hello Word!"
To create our first Rust script we can create a `main.rs` file and add the code to it:
```rust
fn main() {
    println!("Hello, world!");
}
```

Every Rust file must have a main function, which is the entry point to the program.
To run it, first we need to compile and them execute the compiled file:
```rust
rustc main.rs
./main
```

## Comments
In Rust, comments can be added to your code with `//` for single line comments and `/*` for multi-line comments.

## Using Cargo
Cargo is Rust native package manager. Cargo is in charge of essential processes like:
- Managing dependencies for repeatable builds
- Downloading and building external libraries
- Add other responsabilities

To create a new project with cargo just run:
```bash
cargo new <project_name>
```
 
To run Cargo, we need to navigate to the newly created project folder (named as "hello_world") and run:
```bash
cargo run
```
You can check the created files for your cargo project that should look like the structure bellow:
```bash
.
├── Cargo.lock
├── Cargo.toml
├── src
│   └── main.rs
└── target
    ├── CACHEDIR.TAG
    └── debug
        ├── build
        ├── deps
        │   ├── hello_world-495e70b5bc18f55c
        │   └── hello_world-495e70b5bc18f55c.d
        ├── examples
        ├── hello_world
        ├── hello_world.d
        └── incremental
            └── hello_world-2dug673wmaaen
                ├── s-gwftmn96w2-wh65zr-cc4o1kzbpecutpi2i83pxdo1z
                │   ├── 14294eji0esh0ntz.o
                │   ├── 1vzl0wi2hhr02pgx.o
                │   ├── 2il2oaj8o7eelcqp.o
                │   ├── 5fcyojyftykz9l4a.o
                │   ├── dep-graph.bin
                │   ├── l4r19veqlqrkml3.o
                │   ├── prnzv4g3yr6ty75.o
                │   ├── query-cache.bin
                │   └── work-products.bin
                └── s-gwftmn96w2-wh65zr.lock

9 directories, 18 files
```
To generate a release using Cargo you can use the:
```bash
cargo build --release
```
