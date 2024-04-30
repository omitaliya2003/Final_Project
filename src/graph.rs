use std::collections::{HashMap, VecDeque, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};


#[derive(Debug)]
//Storing graph from adjacency list into a HashMap using a struct
pub struct Graph {
    pub adjacency_list: HashMap<usize, Vec<usize>>,
}

impl Graph {
    pub fn num_vertices(&self) -> usize {
        self.adjacency_list.len()
    }
    //Method that uses breadth first search (BFS) to compute shorest path from a source node
    pub fn shortest_paths_from(&self, source: usize) -> HashMap<usize, usize> {
        let mut distances = HashMap::new();
        let mut queue = VecDeque::new();
        let mut visited = HashMap::new();

        queue.push_back(source);
        distances.insert(source, 0);
        visited.insert(source, true);

        while let Some(node) = queue.pop_front() {
            let current_distance = *distances.get(&node).unwrap_or(&0);

            for &neighbor in self.adjacency_list.get(&node).unwrap_or(&vec![]) {
                if !visited.contains_key(&neighbor) {
                    visited.insert(neighbor, true);
                    distances.insert(neighbor, current_distance + 1);
                    queue.push_back(neighbor);
                }
            }
        }

        distances
    }
    
}


// Function to read graph from file and store it into Hash Map 
pub fn read_graph_from_file(filename: &str) -> Option<Graph> {
    let file = File::open(filename).ok()?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    //Use Shuffle rust code to get first random n vertices
    let num_vertices = lines.next()?.unwrap().parse::<usize>().ok()?;
    let mut adjacency_list = HashMap::new();

    for line in lines.filter_map(|line| line.ok()) {
        let mut parts = line.split_whitespace();
        let from = parts.next()?.parse::<usize>().ok()?;
        let to = parts.next()?.parse::<usize>().ok()?;

        adjacency_list.entry(from).or_insert(Vec::new()).push(to);
    }

    Some(Graph { adjacency_list })
}
