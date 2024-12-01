static TEST: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

pub fn main() {
  let test: (Vec<i32>, Vec<i32>) = parse(TEST);
  assert_eq!(pt1(&test), 11);
  println!("Dat 1, Test 1: {}", pt1(&test));

  let input = parse(include_str!("../../data/day1.txt"));
  println!("Day 1, Part 1: {}", pt1(&input));

  assert_eq!(pt2(&test), 31);
  println!("Day 1, Test 2: {}", pt2(&test));

  println!("Day 1, Part 2: {}", pt2(&input));
}

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
  input
    .split("\n")
    .map(|line| {
      let mut parts = line.split_whitespace();
      (
        parts.next().unwrap().parse::<i32>().unwrap(),
        parts.next().unwrap().parse::<i32>().unwrap(),
      )
    })
    .unzip()
}

fn pt1(input: &(Vec<i32>, Vec<i32>)) -> i32 {
  let mut a = input.0.clone();
  let mut b = input.1.clone();
  a.sort();
  b.sort();
  a.iter().zip(b.iter()).map(|(a, b)| (a - b).abs()).sum()
}

fn pt2(input: &(Vec<i32>, Vec<i32>)) -> i32 {
  let mut counts = std::collections::HashMap::new();
  input.1.iter().for_each(|i| {
    let count = counts.entry(i).or_insert(0);
    *count += 1;
  });
  input
    .0
    .iter()
    .map(|i| counts.get(i).unwrap_or(&0) * i)
    .sum()
}
