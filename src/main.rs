mod graph;
mod shortest_paths; // Import the shortest_paths module

use graph::Graph;
use shortest_paths::compute_all_shortest_paths;
use std::collections::HashMap;

fn main() {
    let graph_file = "test_data.txt";
    if let Some(graph) = graph::read_graph_from_file(graph_file) {
        // Compute all shortest paths and store the results in a hashmap
        let all_shortest_paths = compute_all_shortest_paths(&graph);

        // Print all shortest paths stored in the hashmap
        for (source_node, shortest_paths) in &all_shortest_paths {
            println!("Shortest paths from source node {}", source_node);
            for (node, distance) in shortest_paths {
                println!("  Shortest distance to node {} is {}", node, distance);
            }
            println!(); // Print empty line for separation
        }

        // Optionally, you can further process or save the `all_shortest_paths` hashmap here
    } else {
        println!("Failed to read the graph from file.");
    }
}
