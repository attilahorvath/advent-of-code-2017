extern crate day13;

use std::fs::File;
use std::io::{BufRead, BufReader};

use day13::*;

fn main() {
    let file = BufReader::new(File::open("input.txt").expect("file not found"));
    let mut firewall = Firewall::new();

    for l in file.lines() {
        let l = l.expect("error reading file");
        firewall.parse_layer(&l);
    }

    println!("Severity of trip: {}", firewall.trip_severity());
}
