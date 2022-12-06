use regex::Regex;
use std::fs;

pub fn main() {
    let aunts = fs::read_to_string("data/day16.txt").unwrap();

    let aunt1 = find(&aunts, false);
    assert_eq!(aunt1, "Sue 373");
    println!("Day 16: Part 1: the aunt is {}", aunt1);

    let aunt2 = find(&aunts, true);
    assert_eq!(aunt2, "Sue 260");
    println!("Day 16: Part 2: the aunt is {}", aunt2);
}

enum Comparator {
    Equal,
    GreaterThan,
    LessThan,
}

fn find(aunts: &str, part2: bool) -> String {
    let tests = vec![
        ("children", 3, Comparator::Equal),
        ("cats", 7, Comparator::GreaterThan),
        ("samoyeds", 2, Comparator::Equal),
        ("pomeranians", 3, Comparator::LessThan),
        ("akitas", 0, Comparator::Equal),
        ("vizslas", 0, Comparator::Equal),
        ("goldfish", 5, Comparator::LessThan),
        ("trees", 3, Comparator::GreaterThan),
        ("cars", 2, Comparator::Equal),
        ("perfumes", 1, Comparator::Equal),
    ];
    'outer: for aunt in aunts.lines() {
        let mut aunt = aunt.to_string();
        aunt.push(',');

        for test in tests.iter() {
            let match_test = Regex::new(&(" ".to_owned() + test.0 + ": ([0-9]+),")).unwrap();

            match match_test.captures(&aunt) {
                None => continue,
                Some(c) => {
                    let number: i32 = c.get(1).unwrap().as_str().parse().unwrap();

                    let comparison = match part2 {
                        false => &Comparator::Equal,
                        true => &test.2,
                    };

                    match comparison {
                        Comparator::Equal => {
                            if number != test.1 {
                                continue 'outer;
                            }
                        }
                        Comparator::GreaterThan => {
                            if number <= test.1 {
                                continue 'outer;
                            }
                        }
                        Comparator::LessThan => {
                            if number >= test.1 {
                                continue 'outer;
                            }
                        }
                    }
                }
            }
        }
        return aunt.split(": ").next().unwrap().to_string();
    }
    "not found".to_string()
}
