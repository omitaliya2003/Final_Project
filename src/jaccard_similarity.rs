use crate::graph::Graph;
use std::collections::{HashMap, HashSet};

/// Compute Jaccard similarity scores for pairs of vertices with shortest path of 2
/// from different source vertices in the graph.
pub fn compute_jaccard_similarity_for_shortest_paths_of_2(graph: &Graph) -> HashMap<(usize, usize), f64> {
    let mut jaccard_similarities = HashMap::new();

    // Iterate over all source vertices in the graph
    for &source_vertex in graph.adjacency_list.keys() {
        // Compute shortest paths from the current source vertex
        let shortest_paths = graph.shortest_paths_from(source_vertex);

        // Iterate over all vertices in the graph
        for &vertex in graph.adjacency_list.keys() {
            // Check if the shortest path distance from source_vertex to current vertex is 2
            if let Some(distance) = shortest_paths.get(&vertex) {
                if *distance == 2 {
                    // Get neighborhoods (sets of neighboring nodes) for source_vertex and current vertex
                    let neighbors_source: HashSet<usize> = graph.adjacency_list[&source_vertex].iter().cloned().collect();
                    let neighbors_vertex: HashSet<usize> = graph.adjacency_list[&vertex].iter().cloned().collect();

                    // Calculate Jaccard similarity between neighborhoods
                    let intersection_size = neighbors_source.intersection(&neighbors_vertex).count() as f64;
                    let union_size = neighbors_source.union(&neighbors_vertex).count() as f64;
                    let jaccard_similarity = intersection_size / union_size;

                    // Store the computed Jaccard similarity for the vertex pair if above 0.5
                    jaccard_similarities.insert((source_vertex, vertex), jaccard_similarity);
                }
            }
        }
    }

    jaccard_similarities
}
