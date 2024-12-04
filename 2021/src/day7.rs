use num::abs;
use std::fs;

pub fn main() {
  let test = "16,1,2,0,4,2,7,1,2,14"
    .split(',')
    .map(|s| s.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();
  let input = fs::read_to_string("data/day7.txt")
    .unwrap()
    .split(',')
    .map(|s| s.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();

  let test1 = least_fuel(&test, false);
  assert_eq!(test1, (2, 37));
  println!(
    "Day 7: Test 1: Least fuel ({}) used to reach postition {}",
    test1.1, test1.0
  );

  let part1 = least_fuel(&input, false);
  assert_eq!(part1, (342, 325528));
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
  assert_eq!(part2, (460, 85015836));
  println!(
    "Day 7: Part 2: Least fuel ({}) used to reach postition {}",
    part2.1, part2.0
  );
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
