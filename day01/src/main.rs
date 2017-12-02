extern crate day01;

use std::fs::File;
use std::io::prelude::*;

use day01::solve_captcha;

fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect(
        "error reading file",
    );

    println!("{}", solve_captcha(&contents.trim()));
}
