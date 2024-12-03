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
    "(()))",      // Day 1
    "2x3x4",      // Day 2
    "^v^v^v^v^v", // Day 3
    "",           // Day 4
    "ugknbfddgicrmopn
aaa
jchzalrnumimnmhp
haegwjzuvuyypxyu
dvszwmarrgswjxmb", // Day 5
    "turn on 887,9 through 959,629", // Day 6
    "123 -> x
456 -> y
x AND y -> a
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i", // Day 7
    "\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\x27\"", // Day 8
    "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141", // Day 9
    "111221",     // Day 10
    "aaaaaaaa",   // Day 11
    "{\"a\":{\"b\":4},\"c\":-1}", // Day 12
    "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.", // Day 13
    "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.", // Day 14
    "",           // Day 15
    "",           // Day 16
    "",           // Day 17
    "",           // Day 18
    "",           // Day 19
    "",           // Day 20
    "",           // Day 21
    "",           // Day 22
    "",           // Day 23
    "",           // Day 24
    "",           // Day 25
];
