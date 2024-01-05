use std::collections::{HashSet, HashMap};

use itertools::Itertools;
use priority_queue::PriorityQueue;


advent_of_code::solution!(25);

#[derive(Clone)]
struct Graph<'a> {
    nodes: HashSet<&'a str>,
    edges: HashMap<&'a str, HashMap<&'a str, i32>>
}

impl<'a> Graph<'a> {

    fn new() -> Graph<'a> {
        Graph { nodes: HashSet::new(), edges: HashMap::new() }
    }

    fn add_edge(&mut self, a: &'a str, b: &'a str, w: i32) {
        self.nodes.insert(a);
        self.nodes.insert(b);
        self.edges.entry(a).or_insert(HashMap::new()).insert(b, w);
        self.edges.entry(b).or_insert(HashMap::new()).insert(a, w);
    }

    fn remove(&mut self, v: &'a str) {
        self.nodes.remove(v);
        self.edges.remove(v);
        self.edges.iter_mut().for_each(|(_, n)| { n.remove(v); });
    }

    fn update_edge(&mut self, a: &'a str, b: &'a str, w: i32) {
        *self.edges.entry(a).or_default().entry(b).or_insert(0) += w;
        *self.edges.entry(b).or_default().entry(a).or_insert(0) += w;
    }

    fn get_neighbours(&self, n: &'a str) -> Vec<(&'a str, i32)> {
        self.edges.get(n).unwrap().iter().map(|(k, v)| (*k, *v)).collect_vec()
    }

    fn bfs(&self, from: &'a str) -> HashSet<&'a str> {
        let mut visited: HashSet<&str> = HashSet::new();
        let mut q: Vec<&str> = vec![from];

        while let Some(node) = q.pop() {
            if visited.contains(node) {
                continue;
            }

            if let Some(neighbours) = self.edges.get(node) {
                for (neighbour, _) in neighbours {
                    q.push(neighbour);
                }
            }

            visited.insert(node);
        }

        visited
    }
}

impl<'a> std::fmt::Debug for Graph<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "graph TD")?;

        let mut visited = HashSet::new();

        for node in self.nodes.iter() {
            for (destination, weight) in self.edges.get(node).unwrap_or(&HashMap::new()) {
                if !visited.contains(destination) {
                    writeln!(f, "{}---|{}|{}", node, weight, destination)?;
                }
            }
            visited.insert(node);
        }

        Ok(())
    }
}

fn parse_graph(input: &str) -> Graph {
    let mut graph = Graph::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut parts = line.split(": ");

        let source = parts.next().unwrap();
        let destinations = parts.next().unwrap();
        
        for destination in destinations.split_whitespace() {
            graph.add_edge(source, destination, 1);
        }
        
    }
    
    graph
}

fn minimum_cut_phase<'a>(graph: &Graph<'a>) -> (&'a str, &'a str, i32) {
    let mut q = PriorityQueue::new();
    
    for n in graph.nodes.iter() {
        q.push(n, 0);
    }

    let mut s = "";
    let mut t = "";
    let mut w = 0;

    while let Some((n, weight)) = q.pop() {
        s = t;
        t = n;
        w = weight;

        for (e, neighbour_weight) in graph.get_neighbours(n).iter() {
            q.change_priority_by(e, |c| *c += neighbour_weight);
        }
    }
    
    (s, t, w)
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse_graph(input);
    let node_count = input.nodes.len();

    let mut graph = input.clone();

    let mut phases = Vec::new();

    let mut best_phase = 0;
    let mut best_cur_weight = i32::MAX;

    for phase in 0..node_count-1 {
        let (s, t, w) = minimum_cut_phase(&graph);
        phases.push((s, t, w));
        
        if w < best_cur_weight {
            best_cur_weight = w;
            best_phase = phase;
        }

        for (n, w) in graph.get_neighbours(t) {
            graph.update_edge(s, n, w);
        }
        graph.remove(t);
    }

    let mut rebuilt_graph = Graph::new();

    for (a, b, w) in phases.iter().take(best_phase) {
        rebuilt_graph.add_edge(a, b, *w);
    }

    let p1 = rebuilt_graph.bfs(phases[best_phase].1);

    Some((p1.len() * (node_count - p1.len())) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
