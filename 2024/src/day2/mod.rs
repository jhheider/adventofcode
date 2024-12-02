const TEST: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

pub fn main() {
  let test = parse(TEST);
  assert_eq!(pt1(&test), 2);
  println!("Day 2, Test 1: {}", pt1(&test));

  let input = parse(include_str!("../../data/day2.txt"));
  println!("Day 2, Part 1: {}", pt1(&input));

  assert_eq!(pt2(&test), 4);
  println!("Day 2, Test 2: {}", pt2(&test));

  println!("Day 2, Part 2: {}", pt2(&input));
}

fn parse(input: &str) -> Vec<Vec<i32>> {
  input
    .split("\n")
    .map(|line| {
      line
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
    })
    .collect()
}

fn pt1(input: &[Vec<i32>]) -> i32 {
  input.iter().map(|row| check(&diff(row)) as i32).sum()
}

fn pt2(input: &[Vec<i32>]) -> i32 {
  input
    .iter()
    .map(|row| {
      for i in 0..row.len() {
        let mut temp = row.clone();
        temp.remove(i);
        if check(&diff(&temp)) {
          return 1;
        }
      }
      0
    })
    .sum()
}

fn diff(input: &[i32]) -> Vec<i32> {
  let length = input.len();
  let mut result: Vec<i32> = input
    .iter()
    .take(length - 1)
    .zip(input.iter().skip(1))
    .map(|(a, b)| b - a)
    .collect();
  result.sort();
  result
}

fn check(input: &[i32]) -> bool {
  let a = input[0];
  let b = input[input.len() - 1];

  ((-3..=-1).contains(&a) && (-3..=-1).contains(&b))
    || ((1..=3).contains(&a) && (1..=3).contains(&b))
}
