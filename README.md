# odgi-rs

A safe and ergonomic Rust wrapper for the `odgi` dynamic pangenome graph library.

This crate provides high-level bindings that handle the `unsafe` FFI calls internally, allowing you to work with `odgi` graphs using safe, idiomatic Rust.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
odgi = "0.1.0"
```

Then, you can use it in your code:

```rust
use odgi::{Graph, OdgiError};

fn main() -> Result<(), OdgiError> {
    // Load a graph from a file.
    let graph = Graph::load("path/to/your/graph.odgi")?;

    // Use safe methods to interact with the graph.
    println!("Graph has {} nodes.", graph.node_count());

    Ok(())
}
```

## Building

This crate builds `odgi` and its dependencies from source using `cmake`. You will need:
- A C++ compiler that supports C++17 (e.g., `g++` or `clang++`)
- The `cmake` build tool

## License

This crate is licensed under the MIT License, consistent with the `odgi` library itself.