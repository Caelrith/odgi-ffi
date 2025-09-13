// File: src/lib.rs

//! # ODGI FFI
//!
//! A safe and ergonomic Rust wrapper for the `odgi` pangenome graph tool.
//!
//! This crate provides high-level, safe Rust functions to interact with the `odgi`
//! C++ library. It handles the complexities of FFI (Foreign Function Interface)
//! and C++ memory management, allowing you to work with ODGI graphs in an
//! idiomatic Rust fashion.
//!
//! ## Features
//!
//! - **Safe Graph Loading**: Load `.odgi` files into an in-memory `Graph` object
//!   that automatically manages the lifetime of the underlying C++ object.
//! - **File-based Conversions**: Easily convert between `.gfa` and `.odgi` formats.
//! - **Simple API**: Access basic graph properties with safe methods.
//!
//! ## Example
//!
//! Here's a quick example of how to convert a GFA file to ODGI, load it,
//! and get its node count.
//!
//! ```no_run
//! use odgi_ffi::{Graph, gfa_to_odgi, odgi_to_gfa};
//! use std::fs;
//!
//! // Assume "my_graph.gfa" exists.
//! let gfa_path = "my_graph.gfa";
//! let odgi_path = "my_graph.odgi";
//!
//! // 1. Convert the GFA to an ODGI file.
//! gfa_to_odgi(gfa_path, odgi_path).expect("GFA to ODGI conversion failed.");
//!
//! // 2. Load the ODGI graph into memory.
//! let graph = Graph::load(odgi_path).expect("Failed to load ODGI graph.");
//!
//! // 3. Use the safe API to get the node count.
//! let count = graph.node_count();
//! println!("The graph has {} nodes.", count);
//!
//! // Clean up the created file.
//! fs::remove_file(odgi_path).unwrap();
//! ```

mod graph;
mod conversion;

pub use graph::{Graph, Error};
pub use conversion::{gfa_to_odgi, odgi_to_gfa};

#[cxx::bridge(namespace = "odgi")]
mod ffi {
    unsafe extern "C++" {
        include!("odgi-ffi/src/odgi_wrapper.hpp");
        
        type graph_t;

        #[namespace = ""]
        type OpaqueGraph;

        #[namespace = ""]
        fn load_graph(path: &str) -> UniquePtr<OpaqueGraph>;

        #[namespace = ""]
        fn get_graph_t<'a>(graph: &'a OpaqueGraph) -> &'a graph_t;

        #[namespace = ""]
        fn get_node_count(graph: &graph_t) -> u64;
    }
}