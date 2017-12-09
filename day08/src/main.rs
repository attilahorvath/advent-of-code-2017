extern crate day08;

use std::fs::File;
use std::io::{BufRead, BufReader};

use day08::*;

fn main() {
    let file = BufReader::new(File::open("input.txt").expect("file not found"));
    let mut processor = Processor::new();

    for l in file.lines() {
        if let Ok(instruction) = l.expect("error reading file").parse() {
            processor.execute(&instruction);
        }
    }

    println!("Largest register value: {}", processor.largest_value());

    println!(
        "Largest register value ever held: {}",
        processor.largest_value_overall()
    );
}
