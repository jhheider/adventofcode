use std::fs;
use regex::Regex;
use serde_json::Value;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let number = Regex::new("(-?[0-9]+)").unwrap();

    let mut part1 = 0;

    for c in number.find_iter(&input) {
        part1 += c.as_str().parse::<i64>().unwrap();
    }

    assert_eq!(part1, 191164);
    println!("Part 1: total = {}", part1);

    let json: Value = serde_json::from_str(&input).unwrap();

    let part2 = walk_tree(&json);

    assert_eq!(part2, 87842);
    println!("Part 2: total = {}", part2);
}

fn walk_tree(json: &Value) -> i64 {
    match json {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(a) => a.iter().fold(0,|t, j| t + walk_tree(j)),
        Value::Object(o) => {
            match o.values().any(|v| *v == Value::String("red".to_string())) {
                true => 0,
                false => o.values().fold(0, |t, j| t + walk_tree(j)),
            }
        }
    }
}