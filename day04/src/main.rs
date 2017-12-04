extern crate day04;

use std::fs::File;
use std::io::{BufRead, BufReader};

use day04::*;

fn main() {
    let file = BufReader::new(File::open("input.txt").expect("file not found"));
    let passwords = file.lines()
        .map(|l| l.expect("error reading file"))
        .collect::<Vec<_>>();

    println!("Valid passwords: {}", count_valid_passwords(&passwords));
}
