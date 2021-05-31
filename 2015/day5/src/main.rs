use std::fs;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let blacklist = Regex::new(r"ab|cd|pq|xy").unwrap();
    let vowels = Regex::new(r"[aeiou]").unwrap();

    let nice = input.lines()
        .filter(|x| !blacklist.is_match(x))
        .filter(|x| vowels.find_iter(x).count() >= 3)
        .filter(has_double_letter);

    println!("Part 1: {} nice names.", nice.count());

    let nice2 = input.lines()
        .filter(nice2check);

    println!("Part 2: {} nice names.", nice2.count());
}

fn has_double_letter(name: &&str) -> bool {
    let chars: Vec<char> = name.chars().collect();

    for i in 0..(chars.len() - 1) {
        if chars[i] == chars[i + 1] { return true }
    }
    false
}

fn nice2check(name: &&str) -> bool {
    let chars: Vec<char> = name.chars().collect();

    let mut found = false;

    for i in 0..(chars.len() - 3) {
        for j in i + 2..chars.len() - 1 {
            if chars[j] == chars[i] && chars[j + 1] == chars[i + 1] { found = true; }
        }
    }

    if !found { return found }

    for i in 0..(chars.len() - 2) {
        if chars[i] == chars[i + 2] { return true }
    }

    return false
}