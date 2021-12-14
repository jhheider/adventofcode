use std::{collections::HashMap, fs};

pub fn main() {
  let test = parse(
    r"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C",
  );
  let input = parse(&fs::read_to_string("data/day14.txt").unwrap());

  let test1 = run_steps(&test, 10);
  assert_eq!(test1, 1588);
  println!(
    "Day 14: Test 1: Difference in high/low occurances is {}",
    test1
  );

  let part1 = run_steps(&input, 10);
  assert_eq!(part1, 2003);
  println!(
    "Day 14: Part 1: Difference in high/low occurances is {}",
    part1
  );

  let test2 = run_steps(&test, 40);
  assert_eq!(test2, 2188189693529);
  println!(
    "Day 14: Test 2: Difference in high/low occurances is {}",
    test2
  );

  let part2 = run_steps(&input, 40);
  assert_eq!(part2, 2276644000111);
  println!(
    "Day 14: Part 2: Difference in high/low occurances is {}",
    part2
  );
}

fn parse(input: &str) -> (Vec<char>, HashMap<String, char>) {
  let template = input.lines().next().unwrap().chars().collect::<Vec<char>>();
  let rules = input
    .lines()
    .filter_map(|line| {
      if !line.contains(" -> ") {
        return None;
      }
      let mut splits = line.split(" -> ");
      Some((
        splits.next().unwrap().to_string(),
        splits.next().unwrap().chars().next().unwrap(),
      ))
    })
    .collect::<HashMap<String, char>>();
  (template, rules)
}

fn run_steps((input, rules): &(Vec<char>, HashMap<String, char>), steps: usize) -> usize {
  let mut pairs = HashMap::<String, usize>::new();

  for i in 1..input.len() {
    let pair = input[i - 1].to_string() + &input[i].to_string();
    pairs.insert(pair.clone(), pairs.get(&pair).unwrap_or(&0) + 1);
  }

  for _ in 0..steps {
    let mut new_pairs = HashMap::new();
    for (key, value) in pairs.iter() {
      match rules.get(key) {
        None => {
          new_pairs.insert(key.clone(), value + new_pairs.get(key).unwrap_or(&0));
        }
        Some(insert) => {
          let mut letters = key.chars();
          let left = letters.next().unwrap().to_string() + &insert.to_string();
          let right = insert.to_string() + &letters.next().unwrap().to_string();
          new_pairs.insert(left.clone(), new_pairs.get(&left).unwrap_or(&0) + value);
          new_pairs.insert(right.clone(), new_pairs.get(&right).unwrap_or(&0) + value);
        }
      };
    }
    pairs = new_pairs;
  }

  // This part gets a little odd. Since every value appears twice, except the first
  // and last values, we can either get all the first values, or all the last ones.
  // I went with firsts, and so we prime the HashMap with the last character.
  let mut counts = HashMap::from([(*input.last().unwrap(), 1)]);

  for (key, value) in pairs {
    let c = key.chars().next().unwrap();
    counts.insert(c, counts.get(&c).unwrap_or(&0) + value);
  }

  counts.values().max().unwrap() - counts.values().min().unwrap()
}
