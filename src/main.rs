#[macro_use]
extern crate clap;
extern crate ansi_term;

extern crate n_puzzle;

const DEFAULT_HEURISTIC: &'static str = "manhattan";

use ansi_term::Colour::*;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use n_puzzle::{node, astar, parser};
use n_puzzle::heuristic::Heuristic;

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

fn write_to_file(result: &String) {
    let path = Path::new("solution.txt");
    let display = path.display();
    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                   display,
                   Error::description(&why)),
        Ok(file) => file,
    };
    // Write the `result` string to `file`, returns `io::Result<()>`
    match file.write_all(result.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}",
                    display,
                    Error::description(&why)),
        Ok(_) => println!("successfully wrote to {}", display),
    };
    ()
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
        else {
            let goal = node::Goal::new(start.len);
            let mut result = String::new();
            for node in astar::astar(&mut start, &goal, &heuristic).unwrap()
            {
                let node_str = format!("{}\n", node);
                result.push_str(&node_str);
            }
            write_to_file(&result)
        }
    }
}
