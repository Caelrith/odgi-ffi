// File: tests/query_graph_test.rs
use odgi_ffi::{gfa_to_odgi, Graph};
use tempfile::NamedTempFile;

/// A helper function to set up the graph for each test.
/// It converts our new queries.gfa to a temporary ODGI file
/// and loads it into a Graph object.
fn setup_graph() -> (Graph, tempfile::NamedTempFile) {
    let gfa_path = "test_data/queries.gfa";

    // Use a temporary file for the ODGI output to keep tests clean.
    let odgi_temp_file = NamedTempFile::new().expect("Failed to create temp ODGI file");
    let odgi_path = odgi_temp_file.path().to_str().unwrap();

    gfa_to_odgi(gfa_path, odgi_path).expect("Test setup: GFA to ODGI conversion failed");
    let graph = Graph::load(odgi_path).expect("Test setup: Failed to load ODGI graph");

    (graph, odgi_temp_file)
}

#[test]
fn test_get_path_names() {
    let (graph, _temp_file) = setup_graph();

    let mut path_names = graph.get_path_names();
    // The order of path names is not guaranteed, so we sort for a stable comparison.
    path_names.sort();

    assert_eq!(path_names, vec!["x", "y", "z"]);
}

#[test]
fn test_get_node_properties() {
    let (graph, _temp_file) = setup_graph();

    // Test sequence content
    assert_eq!(graph.get_node_sequence(1), "GATTACA");
    assert_eq!(graph.get_node_sequence(4), "GTC");
    
    // Test sequence length
    assert_eq!(graph.get_node_len(1), 7);
    assert_eq!(graph.get_node_len(2), 1);

    // Test a non-existent node
    assert_eq!(graph.get_node_sequence(999), "");
    assert_eq!(graph.get_node_len(999), 0);
}


#[test]
fn test_project_coordinates() {
    let (graph, _temp_file) = setup_graph();
    
    // Project to the start of path 'x' (node 1, offset 0)
    let p1 = graph.project("x", 0).expect("Projection should succeed");
    assert_eq!(p1.node_id, 1);
    assert_eq!(p1.offset, 0);
    assert!(p1.is_forward);
    
    // Project into the middle of the first node on path 'x'
    let p2 = graph.project("x", 5).expect("Projection should succeed");
    assert_eq!(p2.node_id, 1);
    assert_eq!(p2.offset, 5);
    assert!(p2.is_forward);
    
    // Project to the start of the second node on path 'x' (node 2)
    // Node 1 has length 7, so position 7 is the first base of node 2.
    let p3 = graph.project("x", 7).expect("Projection should succeed");
    assert_eq!(p3.node_id, 2);
    assert_eq!(p3.offset, 0);
    assert!(p3.is_forward);

    // Project to the last node on path 'y' (node 4)
    // Path 'y' = 1 (len 7) + 3 (len 1) = path pos 8
    let p4 = graph.project("y", 8).expect("Projection should succeed");
    assert_eq!(p4.node_id, 4);
    assert_eq!(p4.offset, 0);
    assert!(p4.is_forward);
    
    // Test projections that should fail
    assert!(graph.project("x", 100).is_none(), "Position out of bounds should fail");
    assert!(graph.project("nonexistent_path", 0).is_none(), "Non-existent path should fail");
}


#[test]
fn test_get_successors() {
    let (graph, _temp_file) = setup_graph();

    // Node 1 should have two successors: 2 and 3.
    let succs = graph.get_successors(1);
    assert_eq!(succs.len(), 2, "Node 1 should have two successors");
    
    // Check that an edge to node 2 exists with the correct orientations.
    assert!(succs.iter().any(|edge| 
        edge.to_node == 2 && edge.from_orientation == true && edge.to_orientation == true
    ), "Should find edge 1+ -> 2+");
    
    // Check for the edge to node 3.
    assert!(succs.iter().any(|edge|
        edge.to_node == 3 && edge.from_orientation == true && edge.to_orientation == true
    ), "Should find edge 1+ -> 3+");

    // Node 2 has two successors: 2+ -> 4+ and the implicit 2- -> 1-.
    let succs_2 = graph.get_successors(2);
    assert_eq!(succs_2.len(), 2, "Node 2 should have two successors");

    // Check for the edge 2+ -> 4+
    assert!(succs_2.iter().any(|edge|
        edge.to_node == 4 && edge.from_orientation == true && edge.to_orientation == true
    ), "Should find edge 2+ -> 4+");

    // CORRECTED: The bidirected equivalent of 1+ -> 2+ is 2- -> 1-.
    // This means `from_orientation` is false, and `to_orientation` is also false.
    assert!(succs_2.iter().any(|edge|
        edge.to_node == 1 && edge.from_orientation == false && edge.to_orientation == false
    ), "Should find edge 2- -> 1-");
}

#[test]
fn test_get_paths_on_node() {
    let (graph, _temp_file) = setup_graph();

    // Node 1 is on all three paths
    let mut paths_on_1 = graph.get_paths_on_node(1);
    paths_on_1.sort();
    assert_eq!(paths_on_1, vec!["x", "y", "z"]);

    // Node 3 is only on path 'y'
    let paths_on_3 = graph.get_paths_on_node(3);
    assert_eq!(paths_on_3, vec!["y"]);

    // Node 4 is on paths 'x' and 'y'
    let mut paths_on_4 = graph.get_paths_on_node(4);
    paths_on_4.sort();
    assert_eq!(paths_on_4, vec!["x", "y"]);

    // Test a node with no paths
    // (Our GFA doesn't have one, but we can test a non-existent ID)
    let paths_on_999 = graph.get_paths_on_node(999);
    assert!(paths_on_999.is_empty());
}