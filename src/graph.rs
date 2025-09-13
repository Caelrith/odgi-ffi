// File: src/graph.rs

//! Provides the main [`Graph`] struct for in-memory graph operations.
use cxx::UniquePtr;
use std::fmt;
use super::ffi;

// Re-export the FFI data structures so they are part of the public API
// and can be used as return types from the Graph methods.
pub use super::ffi::{Edge, PathPosition};

/// A custom error type for operations within the `odgi-ffi` crate.
///
/// This error is returned by functions that may fail, such as file loading.
/// It contains a `String` with a descriptive error message.
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
/// is automatically and safely deallocated.
///
/// It provides high-level methods to query the graph's properties and structure.
pub struct Graph {
    // This field is private. It holds the pointer to our C++ OpaqueGraph wrapper.
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
    /// Returns an [`Error`] if the C++ backend fails to load the graph, for example
    /// if the file does not exist or is corrupted.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use odgi_ffi::Graph;
    ///
    /// // Assuming "my_graph.odgi" exists and is a valid ODGI file.
    /// let graph = Graph::load("my_graph.odgi").expect("Failed to load graph");
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
    /// use odgi_ffi::Graph;
    ///
    /// let graph = Graph::load("my_graph.odgi").unwrap();
    /// println!("The graph has {} nodes.", graph.node_count());
    /// ```
    pub fn node_count(&self) -> u64 {
        let graph_t_ref = ffi::get_graph_t(&self.inner);
        ffi::get_node_count(graph_t_ref)
    }

    /// Returns a list of all path names in the graph.
    ///
    /// The order of path names is not guaranteed.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use odgi_ffi::Graph;
    ///
    /// let graph = Graph::load("my_graph.odgi").unwrap();
    /// let names = graph.get_path_names();
    /// println!("Paths in graph: {:?}", names);
    /// ```
    pub fn get_path_names(&self) -> Vec<String> {
        let graph_t_ref = ffi::get_graph_t(&self.inner);
        ffi::graph_get_path_names(graph_t_ref)
    }
    
    /// Projects a 0-based linear coordinate on a path to graph coordinates.
    ///
    /// This method finds the specific node, offset, and orientation that correspond
    /// to a given base position along a path.
    ///
    /// # Arguments
    ///
    /// * `path_name` - The name of the path to project onto.
    /// * `pos` - The 0-based nucleotide position on the path.
    ///
    /// # Returns
    ///
    /// Returns `Some(PathPosition)` if the path exists and the position is within
    /// its bounds. Returns `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use odgi_ffi::Graph;
    ///
    /// let graph = Graph::load("my_graph.odgi").unwrap();
    /// if let Some(pos) = graph.project("path1", 100) {
    ///     println!("Position 100 on path1 is at node {}, offset {}", pos.node_id, pos.offset);
    /// } else {
    ///     println!("Position could not be projected.");
    /// }
    /// ```
    pub fn project(&self, path_name: &str, pos: u64) -> Option<PathPosition> {
        let graph_t_ref = ffi::get_graph_t(&self.inner);
        let result_ptr = ffi::graph_project(graph_t_ref, path_name, pos);

        if result_ptr.is_null() {
            None
        } else {
            // Dereference the pointer to get the data and clone it into a 
            // Rust-owned struct. The `PathPosition` struct derives `Clone`.
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
    /// Returns the sequence as a `String`. If the node does not exist, an empty
    /// string is returned.
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
    /// Returns the length of the node's sequence. If the node does not exist,
    /// `0` is returned.
    pub fn get_node_len(&self, node_id: u64) -> u64 {
        let graph_t_ref = ffi::get_graph_t(&self.inner);
        ffi::graph_get_node_len(graph_t_ref, node_id)
    }

    /// Gets all successor edges for a given node ID.
    ///
    /// An "edge" connects the end of one node to the start of another. This function
    /// considers edges leaving both the forward (`+`) and reverse (`-`) orientations
    /// of the source node.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The ID of the source node.
    ///
    /// # Returns
    ///
    /// A `Vec<Edge>` containing all successor edges. Returns an empty vector if the
    /// node does not exist or has no successors.
    pub fn get_successors(&self, node_id: u64) -> Vec<Edge> {
        let graph_t_ref = ffi::get_graph_t(&self.inner);
        ffi::graph_get_successors(graph_t_ref, node_id)
    }

    /// Gets all predecessor edges for a given node ID.
    ///
    /// An "edge" connects the end of one node to the start of another. This function
    /// considers edges arriving at both the forward (`+`) and reverse (`-`)
    /// orientations of the target node.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The ID of the target node.
    ///
    /// # Returns
    ///
    /// A `Vec<Edge>` containing all predecessor edges. Returns an empty vector if the
    /// node does not exist or has no predecessors.
    pub fn get_predecessors(&self, node_id: u64) -> Vec<Edge> {
        let graph_t_ref = ffi::get_graph_t(&self.inner);
        ffi::graph_get_predecessors(graph_t_ref, node_id)
    }

    /// Gets the names of all paths that step on a given node ID.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The ID of the node to query.
    ///
    /// # Returns
    ///
    /// A `Vec<String>` of path names. This may contain duplicates if a path
    /// steps on the same node multiple times. Returns an empty vector if the
    /// node does not exist or has no paths traversing it.
    pub fn get_paths_on_node(&self, node_id: u64) -> Vec<String> {
        let graph_t_ref = ffi::get_graph_t(&self.inner);
        ffi::graph_get_paths_on_node(graph_t_ref, node_id)
    }
}