# ODGI-FFI

[![Crates.io](https://img.shields.io/crates/v/odgi-ffi.svg)](https://crates.io/crates/odgi-ffi)  
[![Docs.rs](https://docs.rs/odgi-ffi/badge.svg)](https://docs.rs/odgi-ffi)  
[![CI](https://github.com/caelrith/odgi-ffi/actions/workflows/ci.yml/badge.svg)](https://github.com/caelrith/odgi-ffi/actions)  
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A safe, ergonomic, and high-performance Rust wrapper for the `odgi` pangenome graph tool.

## Overview

`odgi-ffi` provides idiomatic Rust bindings to the powerful `odgi` C++ library. It allows Rust developers to safely and efficiently load, query, and manipulate pangenome variation graphs. The library handles the complexity of the C++/Rust boundary, offering a clean API that abstracts away unsafe FFI calls.

This crate is ideal for bioinformaticians and developers who want to build high-performance pangenome analysis tools in Rust while leveraging the mature, feature-rich `odgi` ecosystem.

## Features

- **Safe Graph Handling**: Load `.odgi` files into a memory-safe `Graph` struct.
- **Comprehensive Graph Queries**:
  - Get node counts, sequences, and lengths.
  - List all path names and get their total length in base pairs.
  - Traverse the graph by finding node successors and predecessors.
  - Identify which paths step on a specific node or traverse a specific edge.
- **Coordinate Projection**: Project nucleotide positions on a path to their corresponding graph node and offset.
- **File Format Conversion**: Includes utilities to convert between GFA and ODGI formats by leveraging the bundled `odgi` executable.
- **Thread Safety**: The `Graph` object is `Send + Sync`, allowing it to be safely shared across threads for parallel processing.

## Getting Started

### Prerequisites

This crate compiles the `odgi` C++ library from source. To successfully build `odgi-ffi`, you must have a C++ build environment installed on your system. This includes:

- A C++17 compliant compiler (e.g., `g++` or `clang++`)
- `CMake` (version 3.10 or higher)
- `make`

On Debian/Ubuntu, you can install these with:

```bash
sudo apt-get update && sudo apt-get install build-essential cmake
```

## Installation

Add `odgi-ffi` as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
odgi-ffi = "1.0.8"
```

## Ask the AI

The `combined_file.txt` file contains all the source code contained in the `/src` folder. Just copy paste the entire contents of this file in your favorite LLM and ask questions like:

- "Explain how this system works."
- "How can I make an interactive pangenome visualization tool using this crate?"

## Usage

The primary entry point of this library is the `Graph` struct. The typical workflow involves loading an `.odgi` file and then using the `Graph` methods to inspect it.

If your data is in GFA format, you can first convert it using the provided `gfa_to_odgi` function.

### Complete Example

Here is a complete example that demonstrates converting a GFA file, loading the resulting ODGI graph, and performing several queries including the newest functions.

```rust
use odgi_ffi::{gfa_to_odgi, Graph};
use std::io::Write;
use tempfile::NamedTempFile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create a temporary GFA file for demonstration.
    let mut gfa_file = NamedTempFile::new()?;
    writeln!(gfa_file, "H\tVN:Z:1.0")?;
    writeln!(gfa_file, "S\t1\tGATTACA")?; // len 7
    writeln!(gfa_file, "S\t2\tT")?;       // len 1
    writeln!(gfa_file, "L\t1\t+\t2\t+\t0M")?;
    writeln!(gfa_file, "P\tx\t1+,2+\t*")?;
    let gfa_path = gfa_file.path();

    // 2. Define a path for the output ODGI file.
    let odgi_file = NamedTempFile::new()?;
    let odgi_path = odgi_file.path();

    // 3. Convert the GFA file to ODGI format.
    gfa_to_odgi(gfa_path.to_str().unwrap(), odgi_path.to_str().unwrap())?;
    println!("Successfully converted GFA to ODGI.");

    // 4. Load the ODGI graph into memory.
    let graph = Graph::load(odgi_path.to_str().unwrap())?;
    println!("Graph loaded successfully!");

    // 5. Perform queries on the graph.
    assert_eq!(graph.node_count(), 2);
    println!("Node count: {}", graph.node_count());

    let path_names = graph.get_path_names();
    assert_eq!(path_names, vec!["x"]);
    println!("Path names: {:?}", path_names);

    let seq = graph.get_node_sequence(1);
    assert_eq!(seq, "GATTACA");
    println!("Sequence of node 1: {}", seq);

    // Projecting position 7 on path "x" should land at the start of node 2 (0-based).
    let position = graph.project("x", 7).unwrap();
    assert_eq!(position.node_id, 2);
    assert_eq!(position.offset, 0);
    println!("Position 7 on path 'x' projects to node {} at offset {}", position.node_id, position.offset);
    
    // --- DEMONSTRATE NEW FUNCTIONS ---
    
    // Get the length of path "x".
    let length = graph.get_path_length("x").unwrap();
    assert_eq!(length, 8); // GATTACA (7) + T (1)
    println!("Length of path 'x': {} bp", length);

    // Find paths on the edge from node 1 (forward) to node 2 (forward).
    let paths_on_edge = graph.get_paths_on_edge(1, true, 2, true);
    assert_eq!(paths_on_edge, vec!["x"]);
    println!("Paths on edge 1+ -> 2+: {:?}", paths_on_edge);

    Ok(())
}
```

## API Reference

The core of the library is the `odgi_ffi::Graph` struct. Below is a summary of its main methods. For detailed information, please refer to the official documentation on [docs.rs](https://docs.rs/odgi-ffi).

| Method | Description |
|--------|-------------|
| `Graph::load(path)` | Loads an ODGI graph from a file. |
| `node_count()` | Returns the total number of nodes in the graph. |
| `get_path_names()` | Returns a list of all path names. |
| `get_path_length(path)` | Gets the total length of a path in base pairs. |
| `get_node_sequence(id)` | Gets the DNA sequence for a given node ID. |
| `get_node_len(id)` | Gets the length of the sequence for a given node ID. |
| `project(path, pos)` | Projects a linear coordinate on a path to graph coordinates. |
| `get_successors(id)` | Gets all successor edges for a given node. |
| `get_predecessors(id)` | Gets all predecessor edges for a given node. |
| `get_paths_on_node(id)` | Gets the names of all paths that step on a given node. |
| `get_paths_on_edge(...)` | Gets the names of all paths that traverse a specific directed edge. |

## Conversion Utilities

- `gfa_to_odgi(gfa_path, odgi_path)`: Converts a GFA file to an ODGI file.
- `odgi_to_gfa(odgi_path, gfa_path)`: Converts an ODGI file back to a GFA file.

## Building from Source

To build the project locally, clone the repository and use Cargo. Make sure you have the prerequisites installed.

```bash
# Clone the repository
git clone https://github.com/caelrith/odgi-ffi.git
cd odgi-ffi

# Build the project
cargo build --release

# Run tests
cargo test
```

The build script (`build.rs`) automatically handles the following:

- Copies the vendored odgi C++ source code to a temporary build directory.
- Invokes cmake to compile odgi and its dependencies.
- Makes the odgi executable path available as an environment variable (`ODGI_EXE`).
- Configures Cargo to link against the compiled static libraries.
- Builds the C++ FFI wrapper code using `cxx`.

## Contributing

Contributions are welcome! If you find a bug, have a feature request, or want to contribute code, please open an issue or a pull request on our GitHub repository.

## License

This project is licensed under the MIT License.
