// src/lib.rs

// 1. Declare the `graph` module, which corresponds to `src/graph.rs`.
mod graph;

// 2. Re-export the public items from the `graph` module so users can easily
// access them with `use odgi_ffi::Graph;`.
pub use graph::{Graph, Error};


// 3. The FFI bridge is a private implementation detail of this crate.
// Users will not and cannot access this directly.
#[cxx::bridge(namespace = "odgi")]
mod ffi {
    unsafe extern "C++" {
        include!("odgi.hpp");
        include!("odgi-ffi/src/odgi_wrapper.hpp");
        
        // This type is correctly found in the `odgi` namespace.
        type graph_t;

        // The following types and functions are in the global C++ namespace.
        #[namespace = ""]
        type OpaqueGraph;

        #[namespace = ""]
        fn load_graph(path: &str) -> UniquePtr<OpaqueGraph>;

        #[namespace = ""]
        fn get_graph_t<'a>(graph: &'a OpaqueGraph) -> &'a graph_t;

        // CORRECTED: This function also lives in the global namespace, so it needs
        // the attribute to override the module-level `namespace = "odgi"`.
        #[namespace = ""]
        fn get_node_count(graph: &graph_t) -> u64;
    }
}