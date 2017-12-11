extern crate day10;

use std::fs::File;
use std::io::prelude::*;

use day10::*;

fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect(
        "error reading file",
    );

    println!("Hash: {}", hash(contents.trim().as_bytes()));
}
