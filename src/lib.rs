// src/lib.rs
//! A safe Rust interface to the `odgi` C++ library.
//!
//! The `odgi-ffi` crate provides high-level, idiomatic Rust bindings for querying
//! [ODGI](https://github.com/pangenome/odgi) graphs. It handles the complexity of the
//! C++ FFI boundary, providing a safe and easy-to-use API for Rust developers.
//!
//! The primary entry point is the [`Graph`] struct, which represents a loaded ODGI graph
//! in memory. This crate also provides utility functions for converting between GFA and
//! ODGI file formats.
//!
//! # Modules
//!
//! - [`graph`]: Contains the main [`Graph`] struct for querying graph data.
//! - [`conversion`]: Provides functions like [`gfa_to_odgi`] for format conversion.
//!
//! # Features
//!
//! - Load ODGI graphs from disk into a safe Rust wrapper.
//! - Query graph properties, such as node count, path names, and node sequences.
//! - Perform topological queries, such as finding node successors and predecessors.
//! - Project path coordinates to their corresponding nodes and offsets.
//! - Convert between GFA and ODGI formats using the bundled `odgi` executable.
//!
//! # Example
//!
//! Here's a complete example of loading a graph and performing some basic queries.
//!
//! ```rust,no_run
//! use odgi_ffi::{Graph, gfa_to_odgi};
//! use tempfile::NamedTempFile;
//! use std::io::Write;
//!
//! // Create a temporary GFA file for the example.
//! let mut gfa_file = NamedTempFile::new().unwrap();
//! writeln!(gfa_file, "H\tVN:Z:1.0").unwrap();
//! writeln!(gfa_file, "S\t1\tGATTACA").unwrap();
//! writeln!(gfa_file, "S\t2\tT").unwrap();
//! writeln!(gfa_file, "L\t1\t+\t2\t+\t0M").unwrap();
//! writeln!(gfa_file, "P\tx\t1+,2+\t*").unwrap();
//! let gfa_path = gfa_file.path();
//!
//! // Create a path for the ODGI output file.
//! let odgi_file = NamedTempFile::new().unwrap();
//! let odgi_path = odgi_file.path();
//!
//! // 1. Convert the GFA file to an ODGI file.
//! // This function is only available when not using the `docs-only` feature.
//! # #[cfg(not(feature = "docs-only"))]
//! gfa_to_odgi(gfa_path.to_str().unwrap(), odgi_path.to_str().unwrap())
//!      .expect("Failed to convert GFA to ODGI");
//!
//! // 2. Load the ODGI graph into memory.
//! let graph = Graph::load(odgi_path.to_str().unwrap())
//!      .expect("Failed to load ODGI graph");
//!
//! // 3. Query the graph.
//! assert_eq!(graph.node_count(), 2);
//!
//! let path_names = graph.get_path_names();
//! assert_eq!(path_names, vec!["x"]);
//!
//! let seq = graph.get_node_sequence(1);
//! assert_eq!(seq, "GATTACA");
//!
//! // Get path length using the new method.
//! let length = graph.get_path_length("x").unwrap();
//! assert_eq!(length, 8);
//!
//! // Projecting position 7 on path "x" should land at the start of node 2.
//! let position = graph.project("x", 7).unwrap();
//! assert_eq!(position.node_id, 2);
//! assert_eq!(position.offset, 0);
//! ```

mod graph;

// Conditionally compile the conversion module.
// It will not exist for docs.rs builds.
#[cfg(not(feature = "docs-only"))]
mod conversion;

// Publicly re-export the core types for easy access.
pub use graph::{Graph, Error, Edge, PathPosition};

// Conditionally re-export the conversion functions.
#[cfg(not(feature = "docs-only"))]
pub use conversion::{gfa_to_odgi, odgi_to_gfa};


// --- REAL FFI BRIDGE (for normal builds) ---
#[cfg(not(feature = "docs-only"))]
#[cxx::bridge(namespace = "odgi")]
mod ffi {
    /// Represents a directed edge between two nodes in the graph.
    #[derive(Debug, Clone)]
    struct Edge {
        /// The ID of the node this edge points to.
        to_node: u64,
        /// The orientation of the "from" node's handle in this edge.
        from_orientation: bool,
        /// The orientation of the "to" node's handle in this edge.
        to_orientation: bool,
    }

    /// Represents a specific position on a path.
    #[derive(Debug, Clone)]
    struct PathPosition {
        /// The ID of the node at this position.
        node_id: u64,
        /// The 0-based offset within the node's sequence.
        offset: u64,
        /// The orientation of the node on the path at this position.
        is_forward: bool,
    }

    unsafe extern "C++" {
        include!("odgi-ffi/src/odgi_wrapper.hpp");
        include!("odgi-ffi/src/lib.rs.h");

        type graph_t;
        #[namespace = ""]
        type OpaqueGraph;

        #[namespace = ""]
        fn load_graph(path: &str) -> UniquePtr<OpaqueGraph>;
        #[namespace = ""]
        fn get_graph_t<'a>(graph: &'a OpaqueGraph) -> &'a graph_t;
        #[namespace = ""]
        fn get_node_count(graph: &graph_t) -> u64;
        #[namespace = ""]
        fn graph_get_path_names(graph: &graph_t) -> Vec<String>;
        #[namespace = ""]
        fn graph_project(graph: &graph_t, path_name: &str, pos: u64) -> UniquePtr<PathPosition>;
        #[namespace = ""]
        fn graph_get_node_sequence(graph: &graph_t, node_id: u64) -> String;
        #[namespace = ""]
        fn graph_get_node_len(graph: &graph_t, node_id: u64) -> u64;
        #[namespace = ""]
        fn graph_get_successors(graph: &graph_t, node_id: u64) -> Vec<Edge>;
        #[namespace = ""]
        fn graph_get_predecessors(graph: &graph_t, node_id: u64) -> Vec<Edge>;
        #[namespace = ""]
        fn graph_get_paths_on_node(graph: &graph_t, node_id: u64) -> Vec<String>;
        #[namespace = ""]
        fn graph_get_path_length(graph: &graph_t, path_name: &str) -> u64;
        #[namespace = ""]
        fn graph_get_paths_on_edge(
            graph: &graph_t,
            from_node: u64,
            from_orient: bool,
            to_node: u64,
            to_orient: bool
        ) -> Vec<String>;
    }
}

// --- MOCK FFI BRIDGE (for docs.rs) ---
#[cfg(feature = "docs-only")]
mod ffi {
    // This self-contained mock module provides all the types that `graph.rs` needs
    // to compile its public API for documentation purposes.

    // Mock the opaque C++ types.
    pub enum OpaqueGraph {}

    // Provide mock definitions for the shared structs.
    #[derive(Debug, Clone)]
    pub struct Edge {
        pub to_node: u64,
        pub from_orientation: bool,
        pub to_orientation: bool,
    }

    #[derive(Debug, Clone)]
    pub struct PathPosition {
        pub node_id: u64,
        pub offset: u64,
        pub is_forward: bool,
    }
}