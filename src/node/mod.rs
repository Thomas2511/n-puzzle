
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::fmt;

use std::cmp::Ordering;

#[derive(Clone)]
pub struct Node
{
    pub state: Vec<usize>,
    pub len: usize,
    pub score: i32
}

#[derive(Clone)]
pub struct Goal
{
    pub node: Node,
    pub map: HashMap<usize, (usize, usize)>
}

impl Hash for Node
{
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        self.state.hash(state);
    }
}

impl Eq for Node {}

impl Ord for Node
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        other.score.cmp(&(self.score))
    }
}

impl PartialOrd for Node
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node
{
    fn eq(&self, other: &Self) -> bool
    {
        self.state == other.state
    }
}

impl fmt::Display for Node 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let padding = format!("{}", self.len * self.len).len();
        write!(f, "{}" ,(0..self.len * self.len)
               .map(|i| format!("{1:00$}{2}", padding, self.state[i]
                                , if (i + 1) % self.len == 0 { "\n" }
                                else { " " }))
               .collect::<String>())
    }
}

impl fmt::Display for Goal
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", self.node)
    }
}

impl Node
{
    fn swap(&self, sq1: usize, sq2: usize) -> Node
    {
        let mut v = self.state.clone();
        let save = v[sq1];
        v[sq1] = v[sq2];
        v[sq2] = save;
        Node { state: v, len: self.len, score: 0 }
    }

    /// Get all the possible neighbours of a Node
    pub fn get_neighbour(&self) -> Vec<Node>
    {
        let mut res = Vec::with_capacity(4);
        let size = self.len;
        let x = || -> usize
        {
            for i in 0..size*size
            {
                if self.state[i] == 0 { return i; }
            }
            panic!("The empty square is missing");
        }();
        if (x % size) > 0 { res.push(self.swap(x, x - 1)); }
        if x > size { res.push(self.swap(x, x - size)); }
        if ((x + 1) % size) > 0 { res.push(self.swap(x, x + 1)); }
        if (x + size) < (size*size) { res.push(self.swap(x, x + size)); }
        res
    }

    fn get_linear(&self) -> Vec<usize>
    {
        let mut linear: Vec<_> = Vec::new();
        let size: usize = self.len;
        let (mut top, mut down, mut left, mut right) = (0, size - 1, 0, size - 1);
        loop
        {
            for i in left...right { linear.push(self.state[top * size + i]); }
            top += 1;
            if top > down || left > right { break; }
            for i in top...down { linear.push(self.state[i * size + right]); }
            right -= 1;
            if top > down || left > right { break; }
            for i in (left...right).rev() { linear.push(self.state[down * size + i]); }
            down -= 1;
            if top > down || left > right { break; }
            for i in (top...down).rev() { linear.push(self.state[i * size + left]); }
            left += 1;
            if top > down || left > right { break; }
        }
        linear
    }

    /// Checks if a node is solvable
    pub fn is_solvable(&self) -> bool
    {
        let linear: Vec<usize> = self.get_linear();
        let even = { |&x| x % 2 == 0 };
        let mut inversions = 0;
        for (i, el) in linear.iter().enumerate()
        {
            inversions += linear[i + 1 .. linear.len()].iter().fold(0, |sum, &x| if x != 0 && *el > x { sum + 1 } else { sum });
        }
        (even(&inversions)) 
    }
}

impl Goal
{
    // Creates a new Goal
    pub fn new (size: usize) -> Goal
    {
        let mut tab = vec![0; size*size];
        let mut map = HashMap::new();
        let (mut top, mut down, mut left, mut right) = (0, size - 1, 0, size - 1);
        let mut count: usize = 1;
        let fill = |cpt: &usize| if *cpt < (size * size) { *cpt } else { 0 };
        loop
        {
            for i in left...right { map.insert(count, (i, top)); tab[top * size + i] = fill(&count); count += 1; }
            top += 1;
            if top > down || left > right { break; }
            for i in top...down { map.insert(count, (right, i)); tab[i * size + right] = fill(&count); count += 1; }
            right -= 1;
            if top > down || left > right { break; }
            for i in (left...right).rev() { map.insert(count, (i, down)); tab[down * size + i] = fill(&count); count += 1; }
            down -= 1;
            if top > down || left > right { break; }
            for i in (top...down).rev() { map.insert(count, (left, i)); tab[i * size + left] = fill(&count); count += 1;}
            left += 1;
            if top > down || left > right { break; }
        }
        Goal { node: Node { state: tab, len:size, score: 0 }, map: map }
    }
}
