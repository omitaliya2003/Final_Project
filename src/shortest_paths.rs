use crate::graph::Graph;
use std::collections::HashMap;

pub fn compute_all_shortest_paths(graph: &Graph) -> HashMap<usize, HashMap<usize, usize>> {
    let mut all_shortest_paths = HashMap::new();

    // Get all unique source nodes (all keys in the adjacency list)
    let source_nodes: Vec<usize> = graph.adjacency_list.keys().cloned().collect();

    // Iterate over each source node
    for source_node in &source_nodes {
        let mut shortest_paths = HashMap::new();

        // Compute shortest paths from the current source node to all other nodes
        let distances = graph.shortest_paths_from(*source_node);

        // Store shortest paths in the inner hashmap
        for node in graph.adjacency_list.keys() {
            if let Some(distance) = distances.get(node) {
                shortest_paths.insert(*node, *distance);
            }
        }

        // Store shortest paths for the current source node in the outer hashmap
        all_shortest_paths.insert(*source_node, shortest_paths);
    }

    all_shortest_paths
}