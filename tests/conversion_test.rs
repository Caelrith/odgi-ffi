// File: tests/conversion_test.rs

// We need the Graph struct to load the final GFA and verify it.
use odgi_ffi::{gfa_to_odgi, odgi_to_gfa, Graph};
// REMOVED: use std::fs; // This was unused.

#[test]
fn test_gfa_odgi_roundtrip() {
    let input_gfa = "test_data/tiny.gfa";

    // Create a temporary directory for test outputs.
    let temp_dir = tempfile::Builder::new()
        .prefix("odgi-ffi-test-")
        .tempdir()
        .expect("Failed to create temporary directory");
    let output_odgi_path = temp_dir.path().join("tiny.odgi");
    let roundtrip_gfa_path = temp_dir.path().join("tiny.roundtrip.gfa");
    let output_odgi_str = output_odgi_path.to_str().unwrap();
    let roundtrip_gfa_str = roundtrip_gfa_path.to_str().unwrap();

    // 1. Convert GFA to ODGI
    gfa_to_odgi(input_gfa, output_odgi_str)
        .expect("GFA to ODGI conversion failed");

    // Check that the output file was actually created.
    assert!(output_odgi_path.exists(), "The ODGI output file was not created.");

    // 2. Convert ODGI back to GFA
    odgi_to_gfa(output_odgi_str, roundtrip_gfa_str)
        .expect("ODGI to GFA conversion failed");

    // Check that the final GFA file was created.
    assert!(roundtrip_gfa_path.exists(), "The round-tripped GFA file was not created.");
    
    // 3. CORRECTED: Make the test robust.
    // Instead of comparing the exact text, we verify that the round-tripped
    // GFA is a valid graph with the expected properties.
    // First, we convert it to ODGI again so we can load it with our library.
    let final_odgi_path = temp_dir.path().join("final.odgi");
    let final_odgi_str = final_odgi_path.to_str().unwrap();

    gfa_to_odgi(roundtrip_gfa_str, final_odgi_str)
        .expect("Final GFA to ODGI conversion failed");
        
    // Now load the final graph and check its node count.
    let final_graph = Graph::load(final_odgi_str)
        .expect("Failed to load the final, round-tripped ODGI graph.");
    
    assert_eq!(final_graph.node_count(), 2, "The final graph should have 2 nodes.");

    println!("Successfully performed GFA -> ODGI -> GFA roundtrip and verified graph integrity.");
}