# Rust
_The main idea behind this repository is gather all knowledge that i got about rust language._

Rust is a multi-paradigm, high-level and focused on performance and safety. Rust is sintatically similar to C++,


## Dependencies

The website you can find all rust repositories is the [Crates](https://crates.io/). For add dependencies to the project you just have to add it on the **Cargo.toml** file like:
```rust
[dependencies]
ferris-says = "0.2"
```
And after it's added you just run _cargo build_ to install it and add it in the main.rs file like:
```rust
use ferris_says::say;
```
This line means that we can now use the say function that the ferris-says crate exports for us.




## System functions
Functions or resources belonged to Rust ends with the exclamation character, for sample:


```rust
assert_eq!()
println!()
vec![2;3] // repeat 3 times the number 2
```

## Running the project

```bash
cargo run
```
## Basics
* [Data types](basics/datatypes.py)