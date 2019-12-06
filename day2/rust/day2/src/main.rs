extern crate clap;

use std::fs;
use std::io::{BufRead, BufReader};

use clap::{App, Arg};

fn main() {
    let matches = App::new("Day 2 advent of code")
        .version("1.0")
        .author("Gil Hoben")
        .about("Solves problem of day 2 of advent of code")
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

    let mut buf = BufReader::new(file);
    let mut original_text = String::new();

    buf.read_line(&mut original_text)
        .expect("Unable to read line");

    let noun = "12".to_string();
    let verb = "2".to_string();
    let input_text = day2::add_launch_code(original_text.clone(), noun, verb);
    let part1_result = day2::parse(input_text);
    println!(
        "Answer for part1: {:?}",
        part1_result.split(",").next().unwrap()
    );

    let mut part2_result = String::new();
    'outer: for i in 0..=99 {
        'inner: for j in 0..=99 {
            let noun = i.to_string();
            let verb = j.to_string();
            let input_text = day2::add_launch_code(original_text.clone(), noun, verb);
            let result_i = day2::parse(input_text);
            if result_i.split(",").next().unwrap() == "19690720" {
                part2_result = format!("{}", (i * 100 + j).to_string());
                break 'outer;
            }
        }
    }
    println!("{:?}", part2_result);
}
