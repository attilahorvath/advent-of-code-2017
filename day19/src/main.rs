extern crate day19;

use std::fs::File;
use std::io::{BufRead, BufReader};

use day19::*;

fn main() {
    let file = BufReader::new(File::open("input.txt").expect("file not found"));

    let mut map = Map::new();

    for l in file.lines() {
        let row = l.expect("error reading file")
            .chars()
            .map(|c| c.into())
            .collect::<Vec<_>>();

        map.add_row(&row);
    }

    println!("Letters along the path: {}", map.find_path());
}
