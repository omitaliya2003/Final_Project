mod graph;
mod jaccard_similarity;
mod jaccard_stats;

use graph::{Graph, read_graph_from_file};
use jaccard_similarity::compute_jaccard_similarity;
use std::collections::HashMap;
use jaccard_stats::{compute_jaccard_stats, compute_similarity_percentage};

fn main() {
    // Specify the filename of the graph data
    let filename = "final_data.txt";

    // Read the graph from the specified file
    if let Some(graph) = read_graph_from_file(filename) {
        // Compute Jaccard similarity scores for pairs of vertices with shortest path of 2
        let jaccard_similarities = compute_jaccard_similarity(&graph);

        // Print the computed Jaccard similarity scores
        println!("Jaccard Similarity Scores for Pairs of Vertices with Shortest Path of 2:");
        for ((source_vertex, vertex), similarity) in &jaccard_similarities {
            println!("Jaccard Similarity between {} and {}: {:.4}",source_vertex, vertex, similarity);
        }

        let (mean_similarity, max_similarity, max_similarity_vertices) = compute_jaccard_stats(&graph);

        // Print mean and maximum Jaccard similarities
        println!("Mean Jaccard Similarity: {:.4}", mean_similarity);
        println!("Max Jaccard Similarity: {:.4}", max_similarity);

        // Print vertices achieving the maximum similarity score
        println!("Vertices achieving the maximum Jaccard Similarity:");
        for (source_vertex, vertex) in &max_similarity_vertices {
            println!("Vertex 1: {}, Vertex 2: {}", source_vertex, vertex);
        }

        let percentage_ranges = compute_similarity_percentage(&jaccard_similarities);
        println!("Percentage of Vertices with Jaccard Similarity Scores:");
        for (threshold, percentage) in percentage_ranges {
            println!("Threshold: {:.2}, Percentage: {}%", threshold, percentage);
        }
    } else {
        println!("Failed to read the graph from file.");
    }
}


#[test]
fn test_read_graph() {
    if let Some(graph) = read_graph_from_file("test.txt") {
        assert_eq!(graph.num_vertices(), 7)
    } else {
        panic!("Failed to read the graph from file.");
    }
}


#[test]
fn test_compute_jaccard_similarity() {
    if let Some(graph) = read_graph_from_file("test.txt") {
        // Compute Jaccard similarity scores for pairs of vertices with shortest path of 2
        let test_similarities = compute_jaccard_similarity(&graph);
        let expected_similarity = 0.2500;
        assert_eq!(test_similarities[&(1,7)], expected_similarity)
    } else {
        panic!("Failed to read the graph from file.");
    }
}

#[test]
fn test_jaccard_stats_similarity() {
    // Read the graph from the specified file
    if let Some(graph) = read_graph_from_file("test.txt") {
        // Compute Jaccard similarity scores for pairs of vertices with shortest path of 2
        let jaccard_similarities = compute_jaccard_similarity(&graph);

        // Compute mean, maximum Jaccard similarities, and the vertices achieving the maximum score
        let (mean_similarity, max_similarity, max_similarity_vertices) = compute_jaccard_stats(&graph);

        // Perform assertions to validate the results 
        assert_eq!(mean_similarity, 0.2044); 
        assert_eq!(max_similarity, 1.0000); 

    } else {
        panic!("Failed to read the graph from file.");
    }
}

#[test]
fn test_bfs_shortest_path() {
    if let Some(graph) = read_graph_from_file("test.txt") {
        let source_vertex = 1;
        let destination_vertex = 5;
        let shortest_paths = graph.shortest_paths_from(source_vertex);

        // Print shortest path for destination_vertex
        match shortest_paths.get(&destination_vertex) {
            Some(distance) => assert_eq!(*distance, 1) ,
            None => println!("No path found from {} to {}", source_vertex, destination_vertex),
        }
    } else {
        panic!("Failed to read the graph from file.");
    }
}

#[test]
fn test_compute_similarity_percentage() {
    // Read the graph from the test file
    let filename = "test.txt";
    let graph = read_graph_from_file(filename).expect("Failed to read the graph from file.");

    // Compute Jaccard similarity scores for the graph
    let jaccard_similarities = compute_jaccard_similarity(&graph);

    // Compute similarity percentages
    let percentages = compute_similarity_percentage(&jaccard_similarities);

    // Check the correctness of the computed percentages
    assert_eq!(percentages.len(), 11); // 11 thresholds from 0.0 to 1.0
    assert_eq!(percentages[0], (0.0, 42.10526315789473)); 
    assert_eq!(percentages[1], (0.1, 0.0)); 

}