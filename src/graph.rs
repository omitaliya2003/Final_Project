use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};


#[derive(Debug)]
pub struct Graph {
    pub adjacency_list: HashMap<usize, Vec<usize>>,
}

impl Graph {
    pub fn num_vertices(&self) -> usize {
        self.adjacency_list.len()
    }
}

pub fn read_graph_from_file(filename: &str) -> Option<Graph> {
    let file = File::open(filename).ok()?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

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
