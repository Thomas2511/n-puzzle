extern crate regex;

use node::Node;

/// Remove the comments in the file
pub fn remove_comments(lines: Vec<String>) -> Vec<String>
{
    let rgx = regex::Regex::new(r"(#.*)$|(^#.*$)").unwrap();
    lines.iter()
        .map(|x| rgx.replace(x, ""))
        .filter(|x| *x != "")
        .collect()
}

// Converts a string of lines to a Node
pub fn to_node(lines: Vec<String>) -> Node
{
    let len: usize = match lines[0].parse::<usize>() {
        Ok(n) => match n {
            0...2 => panic!("The array size must be equal or greater than 3."),
            _ => n,
        },
        Err(_) => panic!("The array size is invalid."),
    };
    let rgx = regex::Regex::new(r"[\s]+").unwrap();
    let state = rgx.split(&lines[1..].join(" "))
        .filter(|s| *s != "")
        .map(|s| s.parse::<usize>()
             .expect("Wrong file format"))
        .collect::<Vec<usize>>();
    if state.len() != len*len { panic!("Incorrect puzzle size. Expected {}, got {}", len*len, state.len()); }
    Node { state: state, len: len, score: 0 }
}
