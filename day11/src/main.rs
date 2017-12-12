extern crate day11;

use std::fs::File;
use std::io::prelude::*;

use day11::*;

fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect(
        "error reading file",
    );

    let mut coords = HexCoords::new();

    for direction in contents
        .split(',')
        .map(|i| i.parse().expect("error parsing direction"))
        .collect::<Vec<_>>()
    {
        coords.take_step(direction);
    }

    println!("Distance from origin: {}", coords.distance_from_origin());
}
