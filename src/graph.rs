// src/graph.rs

use cxx::UniquePtr;
use std::fmt;

// `super` refers to the parent module (src/lib.rs) and allows us to
// access the private `ffi` module from within our crate.
use super::ffi;

/// A custom error type for operations within the odgi-ffi crate.
#[derive(Debug)]
pub struct Error(String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A safe, idiomatic Rust wrapper around a C++ `odgi::graph_t` object.
///
/// The `Graph` struct manages the lifetime of the underlying C++ graph object.
/// When it is dropped, the C++ object is automatically deallocated.
pub struct Graph {
    // This field is private. It holds the pointer to our C++ OpaqueGraph wrapper.
    inner: UniquePtr<ffi::OpaqueGraph>,
}

impl Graph {
    /// Loads an ODGI graph from a file.
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
        // Unsafe FFI call is encapsulated here.
        let graph_ptr = ffi::load_graph(path);

        if graph_ptr.is_null() {
            Err(Error(format!("Failed to load ODGI graph from '{}'", path)))
        } else {
            Ok(Graph { inner: graph_ptr })
        }
    }

    /// Returns the number of nodes in the graph.
    pub fn node_count(&self) -> u64 {
        // This safe method hides the chain of unsafe FFI calls.
        let opaque_graph_ref = &self.inner;
        let graph_t_ref = ffi::get_graph_t(opaque_graph_ref);
        ffi::get_node_count(graph_t_ref)
    }
}