extern crate day18;

use std::fs::File;
use std::io::{BufRead, BufReader};

use day18::*;

fn main() {
    let file = BufReader::new(File::open("input.txt").expect("file not found"));

    let instructions = file.lines()
        .map(|l| {
            l.expect("error reading file").parse().expect(
                "unable to parse instruction",
            )
        })
        .collect::<Vec<_>>();

    let mut vm = Vm::new(&instructions);

    println!("Recovered frequency: {}", vm.execute());
}
