use crate::data::Data;
use regex::Regex;

pub fn main() {
    let input = Data::get(8).input;

    let part1 = input.lines().fold(0, |a, b| a + decoded_savings(b));

    println!("Day 8: Part 1: {} characters saved.", part1);

    let part2 = input.lines().fold(0, |a, b| a + encoded_gain(b));

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
