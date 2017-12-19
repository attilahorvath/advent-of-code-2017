extern crate day16;

use std::fs::File;
use std::io::prelude::*;

use day16::*;

fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect(
        "error reading file",
    );

    let mut dance = Dance::new(16);

    let dance_moves = contents
        .split(',')
        .map(|s| s.parse().expect("unable to parse dance move"))
        .collect::<Vec<_>>();

    println!(
        "Final order: {}",
        dance.order_after(&dance_moves, 1_000_000_000)
    );
}
