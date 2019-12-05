extern crate clap;

use std::fs;
use std::io::{BufRead, BufReader};

use clap::{App, Arg};

fn main() {
    let matches = App::new("Day 1 advent of code")
        .version("1.0")
        .author("Gil Hoben")
        .about("Solves problem of day 1 of advent of code")
        .arg(
            Arg::with_name("filename")
                .short("f")
                .long("filename")
                .help("Input file")
                .takes_value(true),
        )
        .get_matches();

    let filename = matches.value_of("filename").unwrap();

    let file = fs::File::open(filename).expect("Error reading file");
    let buff = BufReader::new(file);

    let mut result_without_fuel: u32 = 0;
    let mut result_with_fuel: u32 = 0;

    for line in buff.lines() {
        let line = line.expect("Unable to read line");

        let value = line
            .parse::<u32>()
            .expect("Unable to convert string to integer");

        result_without_fuel += day1::compute_fuel_requirement(value);
        result_with_fuel += day1::compute_complete_fuel_requirement(value);
    }

    println!("Total: {}", result_without_fuel);
    println!("Total with fuel: {}", result_with_fuel);
}
