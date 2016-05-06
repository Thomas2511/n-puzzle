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

pub struct Results
{
    pub path: Vec<Node>,
    pub max_states: usize,
    pub total_states: usize,
}

fn reconstruct_path(came_from: &HashMap<Node, Node>, start: &Node) -> Vec<Node>
{
    let mut total_path = vec![start.clone()];
    let mut current = start;
    while let true = came_from.contains_key(current)
    {
        current = came_from.get(current).unwrap();
        total_path.insert(0, current.clone())
    }
    total_path
}

/// Computes the path from the start to the goal using the astar strategy
pub fn astar(start: &mut Node, goal: &Goal, heuristic: &Heuristic, search: &str) -> Option <Results>
{
    let mut closed_set : Vec<Node> = Vec::new();
    let mut opened_set : BinaryHeap<Node> = BinaryHeap::new();
    let mut set_map : HashMap<Node, Set> = HashMap::new();
    let mut came_from : HashMap<Node, Node> = HashMap::new();
    let mut g_score : HashMap<Node, i32> = HashMap::new();
    let mut max_states = 0;
    let mut total_states = 0;
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
                    return Some(Results { path: reconstruct_path(&came_from, &current), max_states: max_states, total_states: total_states })
                }
                closed_set.push(current.clone());
                set_map.insert(current.clone(), Set::Closed);
                for mut neighbour in current.get_neighbour()
                {
                    if let Some(_) = set_map.get(&neighbour) { continue }
                    let ref infinity = MAX;
                    let neighbour_score: i32 = *(g_score.get(&neighbour).unwrap_or_else(|| infinity));
                    let tentative_g_score: i32 = (g_score.get(&current).unwrap()) + 1;
                    match (set_map.get(&neighbour), tentative_g_score >= neighbour_score)
                    {
                        (None, _) =>
                        {
                            neighbour.score = match search {
                                "greedy" => heuristic.get_score(&neighbour, goal),
                                "uniform" => tentative_g_score + heuristic.get_score(&neighbour, goal),
                                _ => panic!("Search type not defined")
                            };
                            came_from.insert(neighbour.clone(), current.clone());
                            g_score.insert(neighbour.clone(), tentative_g_score);
                            opened_set.push(neighbour.clone());
                            total_states += 1;
                            max_states = if opened_set.len() > max_states { opened_set.len() } else { max_states };
                            set_map.insert(neighbour, Set::Opened);
                        }
                        (Some(_), false) =>
                        {
                            neighbour.score = match search {
                                "greedy" => heuristic.get_score(&neighbour, goal),
                                "uniform" => tentative_g_score + heuristic.get_score(&neighbour, goal),
                                _ => panic!("Search type not defined")
                            };
                            came_from.insert(neighbour.clone(), current.clone());
                            g_score.insert(neighbour.clone(), tentative_g_score);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    None
}
