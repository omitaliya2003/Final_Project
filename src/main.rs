mod graph;
mod jaccard_similarity;

use graph::{Graph, read_graph_from_file};
use jaccard_similarity::compute_jaccard_similarity_for_shortest_paths_of_2;
use std::collections::HashMap;

fn main() {
    // Specify the filename of the graph data
    let filename = "final_data1.txt";

    // Read the graph from the specified file
    if let Some(graph) = read_graph_from_file(filename) {
        // Compute Jaccard similarity scores for pairs of vertices with shortest path of 2
        let jaccard_similarities = compute_jaccard_similarity_for_shortest_paths_of_2(&graph);

        // Print the computed Jaccard similarity scores
        println!("Jaccard Similarity Scores for Pairs of Vertices with Shortest Path of 2:");
        for ((source_vertex, vertex), similarity) in &jaccard_similarities {
            println!("Jaccard Similarity between {} and {}: {:.4}",source_vertex, vertex, similarity);
        }
    } else {
        println!("Failed to read the graph from file.");
    }
}
