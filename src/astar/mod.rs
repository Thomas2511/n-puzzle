//! All astar related functions

use std::collections::BinaryHeap;
use std::collections::HashMap;

use std::i32::MAX;

use node::Node;
use node::Goal;

use heuristic::Heuristic;

//use time::PreciseTime;

enum Set
{
    Closed,
    Opened
}

fn reconstruct_path(came_from: &HashMap<Node, Node>, start: &Node) -> Vec<Node>
{
    let mut total_path = vec![start.clone()];
    let mut current = start;
    while let true = came_from.contains_key(current)
    {
        current = came_from.get(current).unwrap();
        total_path.push(current.clone())
    }
    total_path
}

/// Computes the path from the start to the goal using the astar strategy
pub fn astar(start: &mut Node, goal: &Goal, heuristic: &Heuristic) -> Option <Vec<Node>>
{
    let mut closed_set : Vec<Node> = Vec::new();
    let mut opened_set : BinaryHeap<Node> = BinaryHeap::new();
    let mut set_map : HashMap<Node, Set> = HashMap::new();
    let mut came_from : HashMap<Node, Node> = HashMap::new();
    let mut g_score : HashMap<Node, i32> = HashMap::new();
    start.score = heuristic.get_score(start, goal);
    opened_set.push(start.clone());
    set_map.insert(start.clone(), Set::Opened);
    g_score.insert(start.clone(), 0);

    loop
    {
        match opened_set.pop() {
            None => break,
            Some(current) => {
                if current == goal.node {
                    return Some(reconstruct_path(&came_from, &current))
                }
                closed_set.push(current.clone());
                set_map.insert(current.clone(), Set::Closed);
                //let start_time = PreciseTime::now();
                for mut neighbour in current.get_neighbour()
                {
                //    let closed_s_time = PreciseTime::now();
                    if let Some(_) = set_map.get(&neighbour) { continue }
                //    println!("closed find = {}", closed_s_time.to(PreciseTime::now()));
                    let ref infinity = MAX;
                    let neighbour_score: i32 = *(g_score.get(&neighbour).unwrap_or_else(|| infinity));
                    let tentative_g_score: i32 = (g_score.get(&current).unwrap()) + 1;
                //    let match_s_time = PreciseTime::now();
                    match (set_map.get(&neighbour), tentative_g_score >= neighbour_score)
                    {
                        (None, _) =>
                        {
                //            println!("match find = {}", match_s_time.to(PreciseTime::now()));
                //            let match_bis = PreciseTime::now();
                            neighbour.score = heuristic.get_score(&neighbour, goal);
                            came_from.insert(neighbour.clone(), current.clone());
                            g_score.insert(neighbour.clone(), tentative_g_score);
                            opened_set.push(neighbour.clone());
                            set_map.insert(neighbour, Set::Opened);
                //            println!("None Clone = {}", match_bis.to(PreciseTime::now()));
                        }
                        (Some(_), false) =>
                        {
                //            println!("match find = {}", match_s_time.to(PreciseTime::now()));
                //            let match_bis = PreciseTime::now();
                            neighbour.score = heuristic.get_score(&neighbour, goal);
                            came_from.insert(neighbour.clone(), current.clone());
                            g_score.insert(neighbour.clone(), tentative_g_score);
                //            println!("Some Clone = {}", match_bis.to(PreciseTime::now()));
                        }
                        _ => {}
                    }
                }
                //println!("{}", start_time.to(PreciseTime::now()));
            }
        }
    }
    None
}
