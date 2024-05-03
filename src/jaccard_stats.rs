use crate::jaccard_similarity::compute_jaccard_similarity;
use crate::graph::Graph;
use std::collections::{HashMap, HashSet};

/// Compute the mean, maximum Jaccard similarity score, and the vertices achieving the maximum score.
pub fn compute_jaccard_stats(graph: &Graph) -> (f64, f64, Vec<(usize, usize)>) {
    let jaccard_similarities = compute_jaccard_similarity(graph);
    let mut total_similarity = 0.0;
    let mut max_similarity = 0.0;
    let mut max_similarity_vertices = Vec::new();

    for ((source_vertex, vertex), similarity) in &jaccard_similarities {
        total_similarity += *similarity;
        if *similarity > max_similarity {
            max_similarity = *similarity;
            max_similarity_vertices.clear();
            max_similarity_vertices.push((*source_vertex, *vertex));
        } else if *similarity == max_similarity {
            max_similarity_vertices.push((*source_vertex, *vertex));
        }
    }

    let mean_similarity = format!("{:.4}", total_similarity / jaccard_similarities.len() as f64).parse::<f64>().unwrap();

    (mean_similarity, max_similarity, max_similarity_vertices)
}

pub fn compute_similarity_percentage(jaccard_similarities: &HashMap<(usize, usize), f64>) -> Vec<(f64, f64)> {
    let mut percentages = Vec::new();
    let mut current_threshold = 0.0;

    while current_threshold <= 1.0 {
        let mut count_above_threshold = 0;
        let mut total_vertices = 0;

        // Count vertices with similarity score above current_threshold
        for &similarity in jaccard_similarities.values() {
            if similarity >= current_threshold && similarity < current_threshold + 0.1 {
                count_above_threshold += 1;
            }
            total_vertices += 1;
        }

        // Calculate the percentage and store it
        let percentage = (count_above_threshold as f64 / total_vertices as f64) * 100.0;
        percentages.push((current_threshold, percentage));

        // Increment the threshold for the next iteration
        current_threshold += 0.1;
    }

    percentages
}
