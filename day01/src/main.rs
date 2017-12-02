extern crate day01;

use std::fs::File;
use std::io::prelude::*;

use day01::*;

fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect(
        "error reading file",
    );

    println!(
        "First captcha solution: {}",
        solve_captcha(&contents.trim())
    );

    println!(
        "Second captcha solution: {}",
        solve_second_captcha(&contents.trim())
    );
}
