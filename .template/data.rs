use std::{fs, path::PathBuf};

pub struct Data {
    pub input: String,
    pub test: String,
}

impl Data {
    pub fn get(day: usize) -> Data {
        let path = format!("data/day{}.txt", day);
        let test = TESTS[day - 1].to_string();
        if PathBuf::from(&path).exists() {
            Data {
                input: fs::read_to_string(&path).unwrap(),
                test,
            }
        } else {
            Data {
                input: test.clone(),
                test,
            }
        }
    }
}
const TESTS: [&str; 25] = [
    "", // Day 1
    "", // Day 2
    "", // Day 3
    "", // Day 4
    "", // Day 5
    "", // Day 6
    "", // Day 7
    "", // Day 8
    "", // Day 9
    "", // Day 10
    "", // Day 11
    "", // Day 12
    "", // Day 13
    "", // Day 14
    "", // Day 15
    "", // Day 16
    "", // Day 17
    "", // Day 18
    "", // Day 19
    "", // Day 20
    "", // Day 21
    "", // Day 22
    "", // Day 23
    "", // Day 24
    "", // Day 25
];
