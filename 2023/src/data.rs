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
  "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen", // Day 1
  "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", // Day 2
  "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..", // Day 3
  "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", // Day 4
  "", // Day 5
  "Time:      7  15   30
Distance:  9  40  200", // Day 6
  "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483", // Day 7
  "LR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
Zzz = (ZZZ, ZZZ)", // Day 8
  "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45", // Day 9
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
