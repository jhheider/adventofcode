use std::{fs, collections::HashSet};
use regex::Regex;

pub fn main() {
    let test_rules: Vec<(String, String)> = vec![
        ("e".to_string(), "H".to_string()),
        ("e".to_string(), "O".to_string()),
        ("H".to_string(), "HO".to_string()),
        ("H".to_string(), "OH".to_string()),
        ("O".to_string(), "HH".to_string())
    ];
    let test_input = "HOH";
    let test1 = permutations(test_input, &test_rules).len();
    assert_eq!(test1, 4);
    println!("Day 19: Test 1: {} permutations", test1);

    let input_rules = load_rules("data/day19.txt");
    let input = "CRnSiRnCaPTiMgYCaPTiRnFArSiThFArCaSiThSiThPBCaCaSiRnSiRnTiTiMgArPBCaPMgYPTiRnFArFArCaSiRnBPMgArPRnCaPTiRnFArCaSiThCaCaFArPBCaCaPTiTiRnFArCaSiRnSiAlYSiThRnFArArCaSiRnBFArCaCaSiRnSiThCaCaCaFYCaPTiBCaSiThCaSiThPMgArSiRnCaPBFYCaCaFArCaCaCaCaSiThCaSiRnPRnFArPBSiThPRnFArSiRnMgArCaFYFArCaSiRnSiAlArTiTiTiTiTiTiTiRnPMgArPTiTiTiBSiRnSiAlArTiTiRnPMgArCaFYBPBPTiRnSiRnMgArSiThCaFArCaSiThFArPRnFArCaSiRnTiBSiThSiRnSiAlYCaFArPRnFArSiThCaFArCaCaSiThCaCaCaSiRnPRnCaFArFYPMgArCaPBCaPBSiRnFYPBCaFArCaSiAl";

    let part1 = permutations(input, &input_rules).len();
    assert_eq!(part1, 518);
    println!("Day 19: Part 1: {} permutations", part1);

    let test2 = find_solution(test_input) + 1;
    assert_eq!(test2, 3);
    println!("Day 19: Test 2: {} steps to a solution", test2);

    let test3 = find_solution("HOHOHO") + 1;
    assert_eq!(test3, 6);
    println!("Day 19: Test 3: {} steps to a solution", test3);

    let part2 = find_solution(input);
    assert_eq!(part2, 200);
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

fn load_rules(file: &str) -> Vec<(String, String)> {
    let input = fs::read_to_string(file).unwrap();
    input.lines().map(|line| {
        let mut splits = line.split(" => ");
        (splits.next().unwrap().to_owned(), splits.next().unwrap().to_owned())
    }).collect()
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