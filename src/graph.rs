// File: src/graph.rs

//! Provides the main [`Graph`] struct for in-memory graph operations.
//!
//! This module defines the central [`Graph`] object, which is the primary
//! entry point for querying a loaded ODGI graph. It also defines the
//! associated [`Error`] type for handling failures.

use cxx::UniquePtr;
use std::error::Error as StdError;
use std::fmt;
use super::ffi;

// Re-export the FFI data structures so they are part of the public API
// and can be used as return types from the Graph methods.
pub use super::ffi::{Edge, PathPosition};

/// A custom error type for operations within the `odgi-ffi` crate.
///
/// This error is returned by functions that might fail, such as [`Graph::load`].
#[derive(Debug)]
pub struct Error(pub String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl StdError for Error {}

/// A safe, idiomatic Rust wrapper around a C++ `odgi::graph_t` object.
///
/// A `Graph` instance represents a pangenome graph loaded into memory.
/// Once loaded, you can use its methods to perform various queries, such as
/// retrieving node sequences, finding paths, and traversing the graph structure.
///
/// The only way to create a `Graph` is by calling [`Graph::load`].
pub struct Graph {
    // This field is private. It holds the pointer to our C++ OpaqueGraph wrapper.
    inner: UniquePtr<ffi::OpaqueGraph>,
}

impl Graph {
    /// Loads an ODGI graph from a file into memory.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice that holds the path to the ODGI file.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the file does not exist or if the file format is invalid.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use odgi_ffi::Graph;
    ///
    /// match Graph::load("my_graph.odgi") {
    ///     Ok(graph) => println!("Graph loaded successfully!"),
    ///     Err(e) => eprintln!("Failed to load graph: {}", e),
    /// }
    /// ```
    pub fn load(path: &str) -> Result<Self, Error> {
        let graph_ptr = ffi::load_graph(path);
        if graph_ptr.is_null() {
            Err(Error(format!("Failed to load ODGI graph from '{}'", path)))
        } else {
            Ok(Graph { inner: graph_ptr })
        }
    }

    /// Returns the total number of nodes in the graph.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use odgi_ffi::Graph;
    /// # let graph = Graph::load("my_graph.odgi").unwrap();
    /// let count = graph.node_count();
    /// println!("The graph has {} nodes.", count);
    /// ```
    pub fn node_count(&self) -> u64 {
        let graph_t_ref = ffi::get_graph_t(&self.inner);
        ffi::get_node_count(graph_t_ref)
    }

    /// Returns a list of all path names in the graph.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use odgi_ffi::Graph;
    /// # let graph = Graph::load("my_graph.odgi").unwrap();
    /// let paths = graph.get_path_names();
    /// for path_name in paths {
    ///     println!("Found path: {}", path_name);
    /// }
    /// ```
    pub fn get_path_names(&self) -> Vec<String> {
        let graph_t_ref = ffi::get_graph_t(&self.inner);
        ffi::graph_get_path_names(graph_t_ref)
    }

    /// Projects a 0-based linear coordinate on a path to graph coordinates.
    ///
    /// This is useful for finding which node and offset corresponds to a
    /// specific position along a named path.
    ///
    /// # Arguments
    ///
    /// * `path_name` - The name of the path to project onto.
    /// * `pos` - The 0-based nucleotide position along the path.
    ///
    /// # Returns
    ///
    /// Returns `Some(PathPosition)` if the path exists and the position is
    /// within its bounds. Returns `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use odgi_ffi::Graph;
    /// # let graph = Graph::load("my_graph.odgi").unwrap();
    /// if let Some(position) = graph.project("human_chr1", 1_000_000) {
    ///     println!("Position 1M on chr1 is at node {} offset {}",
    ///              position.node_id, position.offset);
    /// } else {
    ///     println!("Position not found on path.");
    /// }
    /// ```
    pub fn project(&self, path_name: &str, pos: u64) -> Option<PathPosition> {
        let graph_t_ref = ffi::get_graph_t(&self.inner);
        let result_ptr = ffi::graph_project(graph_t_ref, path_name, pos);

        if result_ptr.is_null() {
            None
        } else {
            Some(result_ptr.as_ref().unwrap().clone())
        }
    }

    /// Gets the DNA sequence for a given node ID.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The ID of the node to query.
    ///
    /// # Returns
    ///
    /// Returns the sequence as a `String`. If the `node_id` is invalid,
    /// an empty string is returned.
    pub fn get_node_sequence(&self, node_id: u64) -> String {
        let graph_t_ref = ffi::get_graph_t(&self.inner);
        ffi::graph_get_node_sequence(graph_t_ref, node_id)
    }

    /// Gets the length of the sequence for a given node ID.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The ID of the node to query.
    ///
    /// # Returns
    ///
    /// Returns the sequence length. If the `node_id` is invalid, `0` is returned.
    pub fn get_node_len(&self, node_id: u64) -> u64 {
        let graph_t_ref = ffi::get_graph_t(&self.inner);
        ffi::graph_get_node_len(graph_t_ref, node_id)
    }

    /// Gets all successor edges for a given node ID.
    ///
    /// Successors are the nodes immediately following this one in the graph topology.
    pub fn get_successors(&self, node_id: u64) -> Vec<Edge> {
        let graph_t_ref = ffi::get_graph_t(&self.inner);
        ffi::graph_get_successors(graph_t_ref, node_id)
    }

    /// Gets all predecessor edges for a given node ID.
    ///
    /// Predecessors are the nodes immediately preceding this one in the graph topology.
    pub fn get_predecessors(&self, node_id: u64) -> Vec<Edge> {
        let graph_t_ref = ffi::get_graph_t(&self.inner);
        ffi::graph_get_predecessors(graph_t_ref, node_id)
    }

    /// Gets the names of all paths that step on a given node ID.
    pub fn get_paths_on_node(&self, node_id: u64) -> Vec<String> {
        let graph_t_ref = ffi::get_graph_t(&self.inner);
        ffi::graph_get_paths_on_node(graph_t_ref, node_id)
    }
}

/// Marks the `Graph` struct as safe to send between threads.
// The `unsafe` keyword is our guarantee to the compiler that we've ensured
// the underlying C++ object is safe to be sent and accessed across threads,
// which is true in our read-only use case.
unsafe impl Send for Graph {}

/// Marks the `Graph` struct as safe to share between threads.
// The `unsafe` keyword is our guarantee to the compiler that we've ensured
// the underlying C++ object is safe to be sent and accessed across threads,
// which is true in our read-only use case.
unsafe impl Sync for Graph {}