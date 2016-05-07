#[macro_use]
extern crate clap;
extern crate ansi_term;

extern crate n_puzzle;

const DEFAULT_HEURISTIC: &'static str = "manhattan";
const DEFAULT_SEARCH: &'static str = "greedy";

use ansi_term::Colour::*;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use n_puzzle::{node, astar, parser};
use n_puzzle::heuristic::Heuristic;

fn read_file(filename: &String) -> String {
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

fn write_to_file(result: &String, filename: &String) {
    let path = Path::new(filename);
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("Could not create {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };
    match file.write_all(result.as_bytes()) {
        Err(why) => panic!("Could not write to {}: {}", display, Error::description(&why)),
        Ok(_) => println!("Successfully wrote to {}", display),
    }
}

fn solve_puzzle(mut start: node::Node, heuristic: &Heuristic, search: &str)
{
    if !start.is_solvable() { println!("{}", Yellow.bold().paint("This puzzle is not solvable.")); }
    else {
        let goal = node::Goal::new(start.len);
        let mut result_string = String::new();
        let results = astar::astar(&mut start, &goal, &heuristic, &search).unwrap();
        for node in &(results.path)
        {
            let node_str = format!("{}\n", node);
            result_string.push_str(&node_str);
        }
        println!("Total Number of opened states: {}", results.total_states);
        println!("Maximum number of states ever represented in memory: {}", results.max_states);
        println!("Number of moves required: {}", results.path.len() - 1);
        println!("Ordered sequence of states to the solution are to be written to solution.txt ...\n");
        write_to_file(&result_string, &("solution.txt".to_string()))
    }
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let options = clap::App::from_yaml(yaml).get_matches();
    let search = options.value_of("search").unwrap_or(DEFAULT_SEARCH);
    let heuristic = Heuristic::str_to_heuristic(options
                                                .value_of("heuristic")
                                                .unwrap_or(DEFAULT_HEURISTIC)).unwrap();
    let files: Vec<_> = match options.values_of("file") {
        Some(f) => f.collect::<Vec<_>>(),
        None => Vec::new()
    };
    if files.is_empty() {
        let start = node::Node::random_node(3);
        solve_puzzle(start, &heuristic, &search)
    }
    else {
        for file in files {
            let result: String = read_file(&file.to_string());
            let vec: Vec<String> = result.split("\n")
                .map(|s| s.to_string())
                .collect();
            let start = parser::to_node(parser::remove_comments(vec));
            solve_puzzle(start, &heuristic, &search)
        }
    }
}
