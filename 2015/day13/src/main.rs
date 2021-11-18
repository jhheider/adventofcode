use std::fs;
use std::collections::{HashMap, HashSet};
use regex::Regex;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let test = fs::read_to_string("test.txt").unwrap();


    let test = solve(&test, false);
    assert_eq!(test, 330);

    println!("Test: {}", test);

    let part1 = solve(&input, false);
    assert_eq!(part1, 709);

    println!("Part 1: {}", part1);

    let part2 = solve(&input, true);
    assert_eq!(part2, 668);

    println!("Part 2: {}", part2);
}

fn solve(input: &str, part2: bool) -> i64 {
    let parser = Regex::new("(.*) would (gain|lose) ([0-9]+) happiness units by sitting next to (.*)\\.").unwrap();

    let mut happiness = HashMap::new();
    let mut people = HashSet::new();

    for l in input.lines() {
        let caps = parser.captures(l).unwrap();

        let a = caps.get(1).unwrap().as_str();
        let sign = match caps.get(2).unwrap().as_str() {
            "gain" => 1,
            "lose" => -1,
            _ => panic!("Frankly, this is impossible."),
        };
        let magnitude: i64 = caps.get(3).unwrap().as_str().parse().unwrap();
        let b = caps.get(4).unwrap().as_str();

        happiness.insert((a, b), sign * magnitude);
        people.insert(a);
        people.insert(b);

        happiness.insert(("you", a), 0);
        happiness.insert((a, "you"), 0);
        happiness.insert(("you", b), 0);
        happiness.insert((b, "you"), 0);
    }

    if part2 { people.insert("you"); }

    let pivot = people.iter().next().unwrap();
    let mut found = HashSet::<&str>::new();
    found.insert(pivot);

    find_max(&happiness, &people, pivot, pivot, &mut found)
}

fn find_max<'a>(happiness: &HashMap<(&'a str, &'a str), i64>, people: &HashSet<&'a str>, start: &'a str, pivot: &'a str, found: &mut HashSet<&'a str>) -> i64 {
    if found.len() == people.len() {
        return happiness.get(&(pivot, start)).unwrap() + happiness.get(&(start, pivot)).unwrap();
    }

    let mut max = -9999;

    for p in people.iter() {
        if found.contains(p) { continue }

        let p2 = p.to_owned();

        found.insert(p2);
        let mut val = find_max(happiness, people, start, p2, found);
        found.remove(p);

        val += happiness.get(&(pivot, p2)).unwrap() + happiness.get(&(p2, pivot)).unwrap();

        if val > max { max = val; }
    }

    max
}