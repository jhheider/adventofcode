use std::{fs, collections::HashMap};

enum ReducerType {
  O2, CO2
}

pub fn main() {
  let test = r"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010".to_string();
  let input = fs::read_to_string("data/day3.txt").unwrap();

  let test1 = episilon_gamma(&test);
  assert_eq!(test1, 198);
  println!("Day 3: Test 1: power consumption is {}", test1);

  let part1 = episilon_gamma(&input);
  assert_eq!(part1, 3374136);
  println!("Day 3: Part 1: power consumption is {}", part1);

  let test2 = o2_co2(&test);
  assert_eq!(test2, 230);
  println!("Day 3: Test 2: life support rating is {}", test2);

  let part2 = o2_co2(&input);
  assert_eq!(part2, 4432698);
  println!("Day 3: Part 2: life support rating is {}", part2);
}

fn episilon_gamma(input: &str) -> i32 {
  let mut ones = HashMap::<usize, i32>::new();
  let mut length = 0;
  let count = input.lines().count() as i32;

  for line in input.lines() {
    length = line.len();
    for (pos, bit) in line.chars().enumerate() {
      if bit == '1' { ones.insert(pos, ones.get(&pos).unwrap_or(&0) + 1); }
    }
  }

  let mut epsilon = String::new();
  let mut gamma = String::new();

  for pos in 0..length {
    match ones.get(&pos).unwrap() > &(count / 2) {
      true => {
        gamma += "1";
        epsilon += "0";
      },
      false => {
        gamma += "0";
        epsilon += "1";
      }
    }
  }

  let gamma = i32::from_str_radix(&gamma, 2).unwrap();
  let epsilon = i32::from_str_radix(&epsilon, 2).unwrap();

  gamma * epsilon
}

fn o2_co2(input: &str) -> i32 {
  let o2 = i32::from_str_radix(o2_co2_reducer(input, ReducerType::O2), 2).unwrap();
  let co2 = i32::from_str_radix(o2_co2_reducer(input, ReducerType::CO2), 2).unwrap();

  o2 * co2
}

fn o2_co2_reducer(input: &str, rt: ReducerType) -> &str {
  let mut values = input.lines().collect::<Vec<&str>>();

  for pos in 0..values[0].len() {
    if values.len() == 1 { break; }

    let ones = values
      .iter()
      .map(|v| v.chars().collect::<Vec<char>>().get(pos).unwrap().to_owned())
      .filter(|bit| *bit == '1')
      .count();
    let comparison = match rt {
      ReducerType::O2 => ones >= (values.len() + 1) / 2,
      ReducerType::CO2 => ones < (values.len() + 1) / 2,
    };
    let filter_value = match comparison {
      true => '1',
      false => '0',
    };
    values = values.into_iter().filter(|v| v.chars().collect::<Vec<char>>().get(pos).unwrap() == &filter_value).collect();
  }

  values.get(0).unwrap()
}