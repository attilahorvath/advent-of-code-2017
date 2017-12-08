extern crate day07;

use std::fs::File;
use std::io::{BufRead, BufReader};

use day07::*;

fn main() {
    let file = BufReader::new(File::open("input.txt").expect("file not found"));
    let mut tower = Tower::new();

    for l in file.lines() {
        if let Ok(program) = l.expect("error reading file").parse() {
            tower.add(program);
        }
    }

    if let Some(program) = tower.head() {
        println!("Head of tower: {}", program);
    } else {
        println!("No head found");
    }
}
