// File: src/graph.rs

//! Provides the main `Graph` struct for in-memory graph operations.

use cxx::UniquePtr;
use std::fmt;
use super::ffi;

/// A custom error type for operations within the odgi-ffi crate.
///
/// This error is returned by functions that may fail, such as file loading
/// or external command execution. It contains a `String` with a descriptive
/// error message.
#[derive(Debug)]
pub struct Error(pub String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A safe, idiomatic Rust wrapper around a C++ `odgi::graph_t` object.
///
/// This struct manages the lifetime of the underlying C++ graph object using
/// the RAII pattern. When an instance of `Graph` is dropped, the C++ memory
// is automatically and safely deallocated.
pub struct Graph {
    inner: UniquePtr<ffi::OpaqueGraph>,
}

impl Graph {
    /// Loads an ODGI graph from a file into memory.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice that holds the path to the `.odgi` file.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the C++ backend fails to load the graph, for example
    /// if the file does not exist or is corrupted.
    pub fn load(path: &str) -> Result<Self, Error> {
        let graph_ptr = ffi::load_graph(path);
        if graph_ptr.is_null() {
            Err(Error(format!("Failed to load ODGI graph from '{}'", path)))
        } else {
            Ok(Graph { inner: graph_ptr })
        }
    }

    /// Returns the number of nodes in the graph.
    pub fn node_count(&self) -> u64 {
        let opaque_graph_ref = &self.inner;
        let graph_t_ref = ffi::get_graph_t(opaque_graph_ref);
        ffi::get_node_count(graph_t_ref)
    }
}