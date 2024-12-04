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
  "3   4
4   3
2   5
1   3
3   9
3   3", // Day 1
  "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9", // Day 2
  "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))", // Day 3
  "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX", // Day 4
  "",
  "",
  "",
  "",
  "",
  "",
  "",
  "",
  "",
  "",
  "",
  "",
  "",
  "",
  "",
  "",
  "",
  "",
  "",
  "",
  "",
];
