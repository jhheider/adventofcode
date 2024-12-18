use regex::Regex;
use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
};

use crate::data::Data;

pub fn main() {
    let data = Data::get(9);

    let test = solve(&data.test);
    assert_eq!(test, (605, 982));

    let part1 = solve(&data.input);

    println!("Day 9: Test 1: {} - {}", test.0, test.1);
    println!("Day 9: Part 1: {} - {}", part1.0, part1.1);
}

fn solve(list: &str) -> (u32, u32) {
    let mut edges = HashMap::new();
    let mut nodes = HashSet::new();

    for l in list.lines() {
        let r = Regex::new(r"(.*) to (.*) = ([0-9]+)").unwrap();
        let caps = r.captures(l).unwrap();

        let a = caps.get(1).unwrap().as_str();
        let b = caps.get(2).unwrap().as_str();
        let d = caps.get(3).unwrap().as_str().parse().unwrap();

        edges.insert((a, b), d);
        edges.insert((b, a), d);
        edges.insert(("#", a), 0);
        edges.insert(("#", b), 0);
        nodes.insert(a);
        nodes.insert(b);
    }
    solve_rec(&nodes, &edges, "#", HashSet::new())
}

fn solve_rec(
    nodes: &HashSet<&str>,
    edges: &HashMap<(&str, &str), u32>,
    start: &str,
    visited: HashSet<&str>,
) -> (u32, u32) {
    if visited.len() == nodes.len() {
        return (0, 0);
    }

    let mut min0 = u32::MAX;
    let mut max0 = 0;

    for node in nodes.iter() {
        if visited.contains(node) {
            continue;
        }

        let mut visited = visited.clone();

        visited.insert(node);

        let t = solve_rec(nodes, edges, node, visited);
        let t_min = t.0 + edges.get(&(start, *node)).unwrap();
        min0 = min(t_min, min0);

        let t_max = t.1 + edges.get(&(start, *node)).unwrap();
        max0 = max(t_max, max0);
    }

    (min0, max0)
}
