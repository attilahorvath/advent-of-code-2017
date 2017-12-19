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

    for dance_move in contents.split(',') {
        //println!("{}", dance_move);
        dance.step(dance_move.parse().expect("unable to parse dance move"));
    }

    println!("Final order: {}", dance.order());
}
