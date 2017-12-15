extern crate day14;

use std::fs::File;
use std::io::prelude::*;

use day14::*;

fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect(
        "error reading file",
    );

    let grid = Grid::new(&contents.trim());

    println!("Used squares: {}", grid.used_squares());
    println!("Regions: {}", grid.regions());
}
