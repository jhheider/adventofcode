use core::panic;
use std::fs;

pub fn main() {
  let test = parse_input(
    r"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]",
  );
  let input = parse_input(&fs::read_to_string("data/day10.txt").unwrap());

  let test1 = score_corrupted(&test);
  assert_eq!(test1, 26397);
  println!("Day 10: Test 1: corrupted lines score {} points", test1);

  let part1 = score_corrupted(&input);
  assert_eq!(part1, 321237);
  println!("Day 10: Part 1: corrupted lines score {} points", part1);

  let test2 = score_incomplete(&test);
  assert_eq!(test2, 288957);
  println!("Day 10: Test 2: incomplete lines score {} points", test2);

  let part2 = score_incomplete(&input);
  assert_eq!(part2, 2360030859);
  println!("Day 10: Part 2: incomplete lines score {} points", part2);
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
  input.lines().map(|line| line.chars().collect()).collect()
}

fn score_corrupted(input: &[Vec<char>]) -> i32 {
  let mut score = 0;

  'outer: for line in input {
    let mut stack = Vec::<char>::new();
    for c in line {
      match c {
        '[' | '(' | '<' | '{' => stack.push(*c),
        ']' | ')' | '>' | '}' => {
          let last = stack.pop().unwrap();
          match c {
            ')' => {
              if last != '(' {
                score += 3;
                continue 'outer;
              }
            }
            ']' => {
              if last != '[' {
                score += 57;
                continue 'outer;
              }
            }
            '}' => {
              if last != '{' {
                score += 1197;
                continue 'outer;
              }
            }
            '>' => {
              if last != '<' {
                score += 25137;
                continue 'outer;
              }
            }
            _ => panic!("Bad input character: {}", c),
          }
        }
        _ => panic!("Bad input character: {}", c),
      }
    }
  }
  score
}

fn score_incomplete(input: &[Vec<char>]) -> i64 {
  let mut scores = Vec::<i64>::new();

  'outer: for line in input {
    let mut stack = Vec::<char>::new();
    for c in line {
      match c {
        '[' | '(' | '<' | '{' => stack.push(*c),
        ']' | ')' | '>' | '}' => {
          let last = stack.pop().unwrap();
          match c {
            ')' => {
              if last != '(' {
                continue 'outer;
              }
            }
            ']' => {
              if last != '[' {
                continue 'outer;
              }
            }
            '}' => {
              if last != '{' {
                continue 'outer;
              }
            }
            '>' => {
              if last != '<' {
                continue 'outer;
              }
            }
            _ => panic!("Bad input character: {}", c),
          }
        }
        _ => panic!("Bad input character: {}", c),
      }
    }
    stack.reverse();
    scores.push(stack.iter().fold(0, |sum, value| {
      sum * 5
        + match value {
          '(' => 1,
          '[' => 2,
          '{' => 3,
          '<' => 4,
          _ => panic!("Bad input character: {}", value),
        }
    }))
  }
  scores.sort_unstable();

  scores[scores.len() / 2]
}
