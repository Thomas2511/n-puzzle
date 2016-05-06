#[macro_use]
extern crate clap;
extern crate ansi_term;

extern crate n_puzzle;

const DEFAULT_HEURISTIC: &'static str = "manhattan";

use n_puzzle::{node, astar, parser};
use n_puzzle::heuristic::Heuristic;

use ansi_term::Colour::*;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn read_files(filename: &String) -> String {
    let path = Path::new(filename);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("Could not open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };
    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Err(why) => panic!("Could not read {}: {}", display, Error::description(&why)),
        Ok(_) => buffer,
    }
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let options = clap::App::from_yaml(yaml).get_matches();

    let heuristic = Heuristic::str_to_heuristic(options
                                                .value_of("heuristic")
                                                .unwrap_or(DEFAULT_HEURISTIC)).unwrap();
    for file in options.values_of("file").unwrap().collect::<Vec<_>>() {
        let result: String = read_files(&file.to_string());
        let vec: Vec<String> = result.split("\n")
            .map(|s| s.to_string())
            .collect();
        let mut start = parser::to_node(parser::remove_comments(vec));
        if !start.is_solvable() { println!("{}", Yellow.bold().paint("This puzzle is not solvable.")); }
        else { let goal = node::Goal::new(start.len);
            for node in astar::astar(&mut start, &goal, &heuristic).unwrap()
            {
                println!("{}", node)
            }
        }
    }
}
