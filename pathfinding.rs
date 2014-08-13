use std::fmt;
use std::cmp::Eq;
use std::hash::Hash;
use std::collections::{DList, HashMap, HashSet};

pub mod graph {
    use std::cmp::Eq;
    use std::hash::Hash;
    use std::iter::FromIterator;
    use std::collections::HashMap;
    
    pub trait WeightedGraph<'a, T, I: Iterator<(uint, &'a T)>> {
        fn neighbours(&'a self, node: &T) -> I;
    }

    pub struct SimpleGraph<T> {
        edges: HashMap<T, Vec<T>>
    }

    impl<T: Eq + Hash> SimpleGraph<T> {
        pub fn new(edges: HashMap<T, Vec<T>>) -> SimpleGraph<T> {
            SimpleGraph { edges: edges }
        }
    }

    impl<'a, T: Eq + Hash> WeightedGraph<'a, T, Neighbours<'a, T>> for SimpleGraph<T> {
        fn neighbours(&'a self, node: &T) -> Neighbours<'a, T> {
            match self.edges.find(node) {
                Some(vec) => Neighbours { nodes: FromIterator::from_iter(vec.iter().map(|v| (1u, v))) },
                None => Neighbours { nodes: Vec::new() }
            }
        }
    }

    /// A simple interator over a node's neighbours in a weighted graph.
    ///
    /// Each call to `next()` produces a tuple of the edge's weight and a
    /// pointer to the neighbouring node.
    ///
    /// ```rust
    /// use pathfinding::graph::Graph;
    ///
    /// fn print_neighbours<T: std::fmt::Show>(node: &T, graph: &Graph<T>) {
    ///     let neighbours = graph.neighbours(node).collect();
    ///     println!("Neighbours: {}", neighbours);
    /// }
    /// ```
    pub struct Neighbours<'a, T> {
        nodes: Vec<(uint, &'a T)>
    }

    impl<'a, T> Iterator<(uint, &'a T)> for Neighbours<'a, T> {
        fn next(&mut self) -> Option<(uint, &'a T)> {
            self.nodes.pop()
        }

        fn size_hint(&self) -> (uint, Option<uint>) {
            (self.nodes.len(), Some(self.nodes.len()))
        }
    }
}

/// Search exhaustively over the graph, starting at the given node.
///
/// If `goal` is specified, stop searching if it is reached.
pub fn breadth_first_search<'a, T: Eq + Hash + fmt::Show, I: Iterator<(uint, &'a T)>>
    (graph: &'a graph::WeightedGraph<'a, T, I>, start: &'a T, goal: Option<&'a T>) {
    
    let mut frontier = DList::new();
    let mut visited = HashSet::new();
    
    frontier.push(start);
    visited.insert(start);

    loop {
        // Break the loop when we run out of new nodes.
        let current = match frontier.pop() {
            Some(node) => node,
            None => break
        };
        
        println!("    Visiting: {}", current);

        // If `goal` is not None, check if we've reached it and break out
        // early if we have.
        if goal.map_or(false, |g| g.eq(current)) {
            println!("    Goal reached.");
            break;
        }

        for (_, next) in graph.neighbours(current) {
            // Ensure that we only visit each connected node once by
            // keeping track of previously visited nodes.
            if visited.contains(&next) {
                continue;
            } else {
                visited.insert(next);
                frontier.push(next);
            }
        }
    }
}

fn main() {
    let mut map = HashMap::new();
    map.insert("A", vec!("B"));
    map.insert("B", vec!("A", "C", "D"));
    map.insert("C", vec!("A"));
    map.insert("D", vec!("E", "A"));
    map.insert("E", vec!("B"));

    let g = graph::SimpleGraph::new(map);
    
    println!("Searching over the whole graph:");
    breadth_first_search(&g, &"A", None);

    println!("Searching over the graph with goal 'D':");
    breadth_first_search(&g, &"A", Some(&"D"));
}
