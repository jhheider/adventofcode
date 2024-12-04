use crate::data::Data;
use num::abs;

pub fn main() {
  let data = Data::get(7);
  let test = parse(&data.test);
  let input = parse(&data.input);

  let test1 = least_fuel(&test, false);
  assert_eq!(test1, (2, 37));
  println!(
    "Day 7: Test 1: Least fuel ({}) used to reach postition {}",
    test1.1, test1.0
  );

  let part1 = least_fuel(&input, false);
  println!(
    "Day 7: Part 1: Least fuel ({}) used to reach postition {}",
    part1.1, part1.0
  );

  let test2 = least_fuel(&test, true);
  assert_eq!(test2, (5, 168));
  println!(
    "Day 7: Test 2: Least fuel ({}) used to reach postition {}",
    test2.1, test2.0
  );

  let part2 = least_fuel(&input, true);
  println!(
    "Day 7: Part 2: Least fuel ({}) used to reach postition {}",
    part2.1, part2.0
  );
}

fn parse(input: &str) -> Vec<i32> {
  input
    .split(',')
    .map(|s| s.parse::<i32>().unwrap())
    .collect::<Vec<i32>>()
}

fn least_fuel(input: &[i32], part2: bool) -> (i32, i32) {
  let lowest = input.iter().min().unwrap();
  let highest = input.iter().max().unwrap();

  let mut output = (-1, i32::MAX);

  for position in *lowest..=*highest {
    let cost = input.iter().fold(0, |acc, el| {
      if part2 {
        let step = abs(el - position);
        return acc + step * (step + 1) / 2;
      }
      acc + abs(el - position)
    });

    if cost < output.1 {
      output = (position, cost);
    }
  }

  output
}
