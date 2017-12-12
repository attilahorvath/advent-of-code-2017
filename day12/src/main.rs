extern crate day12;

use std::fs::File;
use std::io::{BufRead, BufReader};

use day12::*;

fn main() {
    let file = BufReader::new(File::open("input.txt").expect("file not found"));
    let mut graph = Graph::new();

    for l in file.lines() {
        let l = l.expect("error reading file");
        graph.parse_node(&l);
    }

    println!("Nodes in group 0: {}", graph.nodes_in_group(0).len());
    println!("Groups in the graph: {}", graph.groups().len());
}
