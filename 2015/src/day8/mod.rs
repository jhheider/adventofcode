use regex::Regex;
use std::fs;

pub fn main() {
    let input = fs::read_to_string("data/day8.txt").unwrap();

    let part1 = input.lines().fold(0, |a, b| a + decoded_savings(b));

    assert_eq!(part1, 1333);
    println!("Day 8: Part 1: {} characters saved.", part1);

    let part2 = input.lines().fold(0, |a, b| a + encoded_gain(b));

    assert_eq!(part2, 2046);
    println!("Day 8: Part 2: {} characters added.", part2);
}

fn decoded_savings(s: &str) -> usize {
    let escape = Regex::new(r#"\\([\\"])|\\x([0-9a-zA-Z]{2})"#).unwrap();
    let memory = escape.replace_all(s, "a").to_string();

    s.len() - memory.len() + 2
}

fn encoded_gain(s: &str) -> usize {
    s.matches('\\').count() + s.matches('\"').count() + 2
}
