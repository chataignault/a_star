use ordered_float::OrderedFloat;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State<N> {
    pub cost: OrderedFloat<f64>,
    pub node: N,
}

impl<N> Ord for State<N>
where
    N: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .partial_cmp(&self.cost)
            .unwrap_or(Ordering::Equal)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl<N> PartialOrd for State<N>
where
    N: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn a_star<N, FN, FH, FD>(start: N, goal: N, neighbors: FN, h: FH, d: FD) -> Option<Vec<N>>
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
    open_set.push(State {
        cost: OrderedFloat(h(&start)),
        node: start.clone(),
    });

    while let Some(State { node: current, .. }) = open_set.pop() {
        if current == goal {
            return Some(reconstruct_path(&came_from, &current));
        }

        for neighbor in neighbors(&current) {
            let tentative_g_score =
                g_score.get(&current).unwrap_or(&f64::INFINITY) + d(&current, &neighbor);

            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&f64::INFINITY) {
                came_from.insert(neighbor.clone(), current.clone());
                g_score.insert(neighbor.clone(), tentative_g_score);
                let f = tentative_g_score + h(&neighbor);
                f_score.insert(neighbor.clone(), f);
                open_set.push(State {
                    cost: OrderedFloat(f),
                    node: neighbor,
                });
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
    struct Point(i32, i32);
    #[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
    struct Point3(i32, i32, i32);

    #[test]
    fn test_2d_grid() {
        let start = Point(0, 0);
        let goal = Point(5, 5);

        // Manhattan distance heuristic
        let h = |p: &Point| (goal.0 - p.0).abs() as f64 + (goal.1 - p.1).abs() as f64;

        // Get valid neighbors
        let neighbors = |p: &Point| {
            let mut n = Vec::new();
            for &(dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let x = p.0 + dx;
                let y = p.1 + dy;
                if x.abs() <= goal.0.abs() && y.abs() <= goal.1.abs() {
                    n.push(Point(x, y));
                }
            }
            n
        };

        // Distance between adjacent points is 1.0
        let d = |_: &Point, _: &Point| 1.0;

        let path = a_star(start, goal.clone(), neighbors, h, d);
        assert!(path.is_some());

        let path = path.unwrap();
        assert_eq!(path.len(), 11);
        assert_eq!(path.first(), Some(&Point(0, 0)));
        assert_eq!(path.last(), Some(&Point(5, 5)));
    }

    #[test]
    fn test_3d_grid() {
        let start = Point3(0, 0, 0);
        let goal = Point3(2, 2, 2);

        // Manhattan distance heuristic
        let h = |p: &Point3| {
            (goal.0 - p.0).abs() as f64 + (goal.1 - p.1).abs() as f64 + (goal.2 - p.2).abs() as f64
        };

        // Get valid neighbors
        let neighbors = |p: &Point3| {
            let mut n = Vec::new();
            for &(dx, dy, dz) in &[
                (0, 1, 0),
                (1, 0, 0),
                (0, -1, 0),
                (-1, 0, 0),
                (0, 0, 1),
                (0, 0, -1),
            ] {
                let x = p.0 + dx;
                let y = p.1 + dy;
                let z = p.2 + dz;
                if x >= 0 && x <= 2 && y >= 0 && y <= 2 && z >= 0 && z <= 2 {
                    n.push(Point3(x, y, z));
                }
            }
            n
        };

        // Distance between adjacent points is 1.0
        let d = |_: &Point3, _: &Point3| 1.0;

        let path = a_star(start, goal.clone(), neighbors, h, d);
        assert!(path.is_some());

        let path = path.unwrap();
        assert_eq!(path.len(), 7);
        assert_eq!(path.first(), Some(&Point3(0, 0, 0)));
        assert_eq!(path.last(), Some(&Point3(2, 2, 2)));
    }
}
