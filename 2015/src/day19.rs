use regex::Regex;
use std::collections::HashSet;

use crate::data::Data;

pub fn main() {
    let data = Data::get(19);
    let (test_rules, test_input) = load_rules(&data.test);
    let (input_rules, input) = load_rules(&data.input);

    let test1 = permutations(test_input, &test_rules).len();
    assert_eq!(test1, 4);
    println!("Day 19: Test 1: {} permutations", test1);

    let part1 = permutations(input, &input_rules).len();
    println!("Day 19: Part 1: {} permutations", part1);

    let test2 = find_solution(test_input) + 1;
    assert_eq!(test2, 3);
    println!("Day 19: Test 2: {} steps to a solution", test2);

    let test3 = find_solution("HOHOHO") + 1;
    assert_eq!(test3, 6);
    println!("Day 19: Test 3: {} steps to a solution", test3);

    let part2 = find_solution(input);
    println!("Day 19: Part 2: {} steps to a solution", part2);
}

fn permutations(input: &str, rules: &[(String, String)]) -> HashSet<String> {
    let mut results = HashSet::new();

    for (find, replacement) in rules.iter() {
        let r = Regex::new(find).unwrap();

        for found in r.find_iter(input) {
            let lead = input.to_string()[0..found.start()].to_string();
            let tail = input.to_string()[found.end()..].to_string();
            results.insert(lead + replacement + tail.as_str());
        }
    }
    results
}

fn load_rules(input: &str) -> (Vec<(String, String)>, &str) {
    let mut res = (vec![], "");
    for line in input.lines() {
        if line.contains("=>") {
            let mut splits = line.split(" => ");
            res.0.push((
                splits.next().unwrap().to_owned(),
                splits.next().unwrap().to_owned(),
            ));
        } else {
            res.1 = line;
        }
    }
    res
}

fn find_solution(input: &str) -> usize {
    // Rules are a grammar. There are 3 possibilities:
    // X => XX (1 step, +1 length)
    // X => X Rn X Ar (1 step, +3 length)
    // X => X Rn X Y X Ar (1 step, +5 length)
    // X => X Rn X Y X Y X Ar (1 step, +7 length)
    // So, using simple math, we can tokenize the string and compute the steps as:
    // X - #Rn - #Ar - 2 * #Y - 1
    let x = Regex::new(r"[A-Z][a-z]?").unwrap();
    let tokens = x.find_iter(input).count();

    let rn = Regex::new(r"Rn").unwrap();
    let rn_tokens = rn.find_iter(input).count();

    let y = Regex::new(r"Y").unwrap();
    let y_tokens = y.find_iter(input).count();

    tokens - 2 * rn_tokens - 2 * y_tokens - 1
}
