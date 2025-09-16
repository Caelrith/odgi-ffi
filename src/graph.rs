//! Provides the main [`Graph`] struct for in-memory graph operations.
use cxx::UniquePtr;
use std::fmt;
use super::ffi;
use std::error::Error as StdError;

// Re-export the FFI data structures so they are part of the public API
// and can be used as return types from the Graph methods.
pub use super::ffi::{Edge, PathPosition};

/// A custom error type for operations within the `odgi-ffi` crate.
#[derive(Debug)]
pub struct Error(pub String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl StdError for Error {}

/// A safe, idiomatic Rust wrapper around a C++ `odgi::graph_t` object.
pub struct Graph {
    // This field is private. It holds the pointer to our C++ OpaqueGraph wrapper.
    inner: UniquePtr<ffi::OpaqueGraph>,
}

impl Graph {
    /// Loads an ODGI graph from a file into memory.
    pub fn load(path: &str) -> Result<Self, Error> {
        let graph_ptr = ffi::load_graph(path);
        if graph_ptr.is_null() {
            Err(Error(format!("Failed to load ODGI graph from '{}'", path)))
        } else {
            Ok(Graph { inner: graph_ptr })
        }
    }
    
    /// Returns the total number of nodes in the graph.
    pub fn node_count(&self) -> u64 {
        let graph_t_ref = ffi::get_graph_t(&self.inner);
        ffi::get_node_count(graph_t_ref)
    }

    /// Returns a list of all path names in the graph.
    pub fn get_path_names(&self) -> Vec<String> {
        let graph_t_ref = ffi::get_graph_t(&self.inner);
        ffi::graph_get_path_names(graph_t_ref)
    }
    
    /// Projects a 0-based linear coordinate on a path to graph coordinates.
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
    pub fn get_node_sequence(&self, node_id: u64) -> String {
        let graph_t_ref = ffi::get_graph_t(&self.inner);
        ffi::graph_get_node_sequence(graph_t_ref, node_id)
    }

    /// Gets the length of the sequence for a given node ID.
    pub fn get_node_len(&self, node_id: u64) -> u64 {
        let graph_t_ref = ffi::get_graph_t(&self.inner);
        ffi::graph_get_node_len(graph_t_ref, node_id)
    }

    /// Gets all successor edges for a given node ID.
    pub fn get_successors(&self, node_id: u64) -> Vec<Edge> {
        let graph_t_ref = ffi::get_graph_t(&self.inner);
        ffi::graph_get_successors(graph_t_ref, node_id)
    }

    /// Gets all predecessor edges for a given node ID.
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

// NEW: Implement thread-safety traits for our FFI wrapper.
// The `unsafe` keyword is our guarantee to the compiler that we've ensured
// the underlying C++ object is safe to be sent and accessed across threads,
// which is true in our read-only use case.
unsafe impl Send for Graph {}
unsafe impl Sync for Graph {}