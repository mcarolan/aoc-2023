use std::collections::{HashSet, HashMap};

use itertools::Itertools;


advent_of_code::solution!(25);

struct Graph<'a> {
    nodes: HashSet<&'a str>,
    edges: HashMap<&'a str, HashMap<&'a str, i32>>
}

impl<'a> Graph<'a> {
    fn compact(&self, phase: &MinimumCutPhase) -> Graph<'a> {
        let mut edges = self.edges.clone();
        let mut nodes = self.nodes.clone();

        if let Some(s_edges) = edges.get_mut(phase.s) {
            for (to, weight) in self.edges.get(phase.t).unwrap() {
                let existing = s_edges.get(to).unwrap_or(&0);
                s_edges.insert(to, *existing + *weight);
            }
        }
     
        nodes.remove(phase.t);

        Graph { nodes, edges }
    }

    fn from_phases(phases: &'a Vec<&MinimumCutPhase>) -> Graph<'a> {
        let mut nodes: HashSet<&str> = HashSet::new();
        let mut edges: HashMap<&str, HashMap<&str, i32>> = HashMap::new();

        for phase in phases.iter() {
            nodes.insert(phase.s);
            nodes.insert(phase.t);

            if !edges.contains_key(phase.s) {
                edges.insert(phase.s, HashMap::new());
            }
            if !edges.contains_key(phase.t) {
                edges.insert(phase.t, HashMap::new());
            }

            if let Some(s_edges) = edges.get_mut(phase.s) {
                s_edges.insert(phase.t, phase.w);
            }
            if let Some(t_edges) = edges.get_mut(phase.t) {
                t_edges.insert(phase.t, phase.w);
            }
        }
        
        Graph { nodes, edges } 
    }

    fn bfs(&'a self, from: &'a str) -> HashSet<&'a str> {
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
    let mut nodes = HashSet::new();
    let mut edges = HashMap::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut parts = line.split(": ");

        let source = parts.next().unwrap();
        let destinations = parts.next().unwrap();

        nodes.insert(source);
        
        for destination in destinations.split_whitespace() {
            nodes.insert(destination);

            if !edges.contains_key(source) {
                edges.insert(source, HashMap::new());
            }
            if !edges.contains_key(destination) {
                edges.insert(destination, HashMap::new());
            }

            if let Some(s_edges) = edges.get_mut(source) {
                s_edges.insert(destination, 1);
            }
            if let Some(d_edges) = edges.get_mut(destination) {
                d_edges.insert(source, 1);
            }
        }
        
    }
    Graph { nodes, edges }
}

#[derive(Debug, Clone)]
struct MinimumCutPhase<'a> {
    s: &'a str,
    t: &'a str,
    w: i32
}

fn minimum_cut_phase<'a>(graph: &Graph<'a>) -> MinimumCutPhase<'a> {
    let start = **graph.nodes.iter().collect_vec().first().unwrap();

    let mut a: HashSet<&str> = HashSet::from_iter(vec![start]);
    let mut remaining: HashSet<&str> = graph.nodes.clone();
    remaining.remove(start);

    let mut visit_order = vec![ start ];
    let mut w = i32::MIN;

    while !remaining.is_empty() {
        let mut tightest: Option<&str> = None;
        let mut tightest_weight = i32::MIN;

        for node in remaining.iter() {
            let mut tightness = 0;
            for (destination, weight) in graph.edges.get(node).unwrap() {
                if a.contains(destination) {
                    tightness += weight;
                }
            }

            if tightness > tightest_weight {
                tightest_weight = tightness;
                tightest = Some(node);
            }
        }

        a.insert(tightest.unwrap());
        remaining.remove(tightest.unwrap());
        visit_order.push(tightest.unwrap());
        w = tightest_weight;
    }
    let t = visit_order.pop().unwrap();
    let s = visit_order.pop().unwrap();

    MinimumCutPhase { s, t, w }
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse_graph(input);
    let node_count = input.nodes.len();

    let mut graph = input;

    let mut phases = Vec::new();

    while graph.nodes.len() > 1 {
        let phase = minimum_cut_phase(&graph);
        phases.push(phase.clone());
        graph = graph.compact(&phase.clone());
    }

    let (i, min_phase) = phases.iter().enumerate().min_by_key(|(_, p)| p.w).unwrap();
    let to_build_with = phases.iter().take(i + 1).collect_vec();
    let rebuilt_graph = Graph::from_phases(&to_build_with);

    let p1 = rebuilt_graph.bfs(min_phase.s);

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
