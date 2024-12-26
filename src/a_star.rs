use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Ordering;
use std::hash::Hash;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State<N> {
    cost: f64,
    node: N,
}

impl<N> Ord for State<N> where N: Ord {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost)
            .unwrap_or(Ordering::Equal)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl<N> PartialOrd for State<N> where N: Ord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn a_star<N, FN, FH, FD>(
    start: N,
    goal: N,
    neighbors: FN,
    h: FH,
    d: FD,
) -> Option<Vec<N>>
where
    N: Eq + Hash + Clone + Ord,
    FN: Fn(&N) -> Vec<N>,
    FH: Fn(&N) -> f64,
    FD: Fn(&N, &N) -> f64,
{
    let mut open_set = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();
    let mut f_score = HashMap::new();
    
    g_score.insert(start.clone(), 0.0);
    f_score.insert(start.clone(), h(&start));
    open_set.push(State { cost: h(&start), node: start.clone() });
    
    while let Some(State { node: current, .. }) = open_set.pop() {
        if current == goal {
            return Some(reconstruct_path(&came_from, &current));
        }
        
        for neighbor in neighbors(&current) {
            let tentative_g_score = g_score.get(&current).unwrap_or(&f64::INFINITY) + d(&current, &neighbor);
            
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&f64::INFINITY) {
                came_from.insert(neighbor.clone(), current.clone());
                g_score.insert(neighbor.clone(), tentative_g_score);
                let f = tentative_g_score + h(&neighbor);
                f_score.insert(neighbor.clone(), f);
                open_set.push(State { cost: f, node: neighbor });
            }
        }
    }
    
    None
}

fn reconstruct_path<N: Clone + Eq + Hash>(came_from: &HashMap<N, N>, current: &N) -> Vec<N> {
    let mut path = vec![current.clone()];
    let mut current = current;
    
    while let Some(prev) = came_from.get(current) {
        path.push(prev.clone());
        current = prev;
    }
    
    path.reverse();
    path
}

