use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum Signal {
    Value(String),
    And(String, String),
    Or(String, String),
    LShift(String, usize),
    RShift(String, usize),
    Not(String),
}

pub fn main() {
    let input = fs::read_to_string("data/day7.txt").unwrap();

    let mut wires = parse_input(input);
    let mut cache = HashMap::new();

    let part1 = get_value(&wires, "a", &mut cache);

    assert_eq!(part1, 956);
    println!("Day 7: Part 1: a wire is: {}", part1);

    cache.clear();
    wires.insert("b".to_string(), Signal::Value(part1.to_string()));

    let part2 = get_value(&wires, "a", &mut cache);

    assert_eq!(part2, 40149);
    println!("Day 7: Part 2: a wire is: {}", part2);
}

fn parse_input(input: String) -> HashMap<String, Signal> {
    let signal = Regex::new(r"^([0-9a-z]+) -> ([a-z]+)$").unwrap();
    let binary =
        Regex::new(r"^([a-z0-9]+) (AND|OR|LSHIFT|RSHIFT) ([a-z0-9]+) -> ([a-z]+)$").unwrap();
    let unary = Regex::new(r"^NOT ([a-z]+) -> ([a-z]+)$").unwrap();

    let mut wires = HashMap::new();

    for line in input.lines() {
        if let Some(c) = signal.captures(line) {
            let value = c.get(1).unwrap().as_str().to_string();
            let wire = c.get(2).unwrap().as_str().to_string();

            wires.insert(wire, Signal::Value(value));
        } else if let Some(c) = binary.captures(line) {
            let left = c.get(1).unwrap().as_str().to_string();
            let right = c.get(3).unwrap().as_str().to_string();
            let wire = c.get(4).unwrap().as_str().to_string();
            match c.get(2).unwrap().as_str() {
                "AND" => wires.insert(wire, Signal::And(left, right)),
                "OR" => wires.insert(wire, Signal::Or(left, right)),
                "LSHIFT" => wires.insert(wire, Signal::LShift(left, right.parse().unwrap())),
                "RSHIFT" => wires.insert(wire, Signal::RShift(left, right.parse().unwrap())),
                &_ => panic!("Unknown operation"),
            };
        } else if let Some(c) = unary.captures(line) {
            let left = c.get(1).unwrap().as_str().to_string();
            let wire = c.get(2).unwrap().as_str().to_string();

            wires.insert(wire, Signal::Not(left));
        }
    }
    wires
}

fn get_value(wires: &HashMap<String, Signal>, key: &str, cache: &mut HashMap<String, u16>) -> u16 {
    let value = match key.parse() {
        Ok(value) => value,
        Err(_) => {
            if let Some(value) = cache.get(key) {
                return *value;
            }
            match wires.get(key).unwrap() {
                Signal::Value(value) => get_value(wires, value, cache),
                Signal::And(left, right) => {
                    get_value(wires, left, cache) & get_value(wires, right, cache)
                }
                Signal::Or(left, right) => {
                    get_value(wires, left, cache) | get_value(wires, right, cache)
                }
                Signal::LShift(left, shift) => get_value(wires, left, cache) << shift,
                Signal::RShift(left, shift) => get_value(wires, left, cache) >> shift,
                Signal::Not(left) => !get_value(wires, left, cache),
            }
        }
    };
    cache.insert(key.to_string(), value);
    value
}
