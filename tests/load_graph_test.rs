// tests/load_graph_test.rs

use odgi_ffi::Graph;

#[test]
fn test_safe_graph_api() {
    let graph_path = "test_data/tiny.odgi";

    // 1. Call the safe `load` function.
    //    .unwrap() is fine in tests, as it will cause a panic on failure.
    let graph = Graph::load(graph_path).unwrap();
    println!("Successfully loaded graph using the safe API.");

    // 2. Call the safe `node_count` method.
    let count = graph.node_count();
    println!("Graph has {} nodes.", count);

    // 3. Assert that the result is correct for our test graph.
    assert_eq!(count, 2, "The node count should be 2 for the test graph.");
    println!("Node count is correct.");
}