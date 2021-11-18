use std::char;
use fancy_regex::*;

fn main() {
    let mut input = String::from("hxbxwxba");

    let part1 = loop {
      input = bump(input);
      if is_valid(&input) { break input; }
    };
    assert_eq!(part1, "hxbxxyzz");
    println!("Part 1: next password is {}", part1);

    input = part1;
    // This is definitely slow if you don't use `cargo run --release`
    let part2 = loop {
        input = bump(input);
        if is_valid(&input) { break input; }
    };

    assert_eq!(part2, "hxcaabcc");
    println!("Part 2: next password is {}", part2);
}

fn bump(pw: String) -> String {
    let mut carry = false;
    pw.chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            if i == 0 || carry {
                carry = false;
                return match c {
                    'h' | 'k' | 'n' => char::from_u32(c as u32 + 2).unwrap(),
                    'z' => {
                        carry = true;
                        'a'
                    },
                    _ => char::from_u32(c as u32 + 1).unwrap(),
                }
            }
            c
        })
        .collect::<String>()
        .chars()
        .rev()
        .collect()
}

fn is_valid(pw: &str) -> bool {
    let bad_letters = Regex::new("[iol]").unwrap();
    let match_two = Regex::new(r"(.)\1.*(.)\2").unwrap();

    if bad_letters.is_match(pw).unwrap() { return false }
    if !match_two.is_match(pw).unwrap() { return false }

    let mut run = false;
    let chars = pw.as_bytes();
    for i in 0..6 {
        if chars[i + 1] == chars[i] + 1 && chars[i + 2] == chars[i] + 2 { run = true; }
    }
    if !run { return false }
    true
}