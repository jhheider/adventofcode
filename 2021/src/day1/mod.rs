use std::fs;

pub fn main() {
  let test = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
  let data: Vec<i32> = fs::read_to_string("data/day1.txt")
    .unwrap()
    .lines()
    .map(|line| line.parse::<i32>().unwrap())
    .collect();

  let test_out = part1(&test);
  assert_eq!(test_out, 7);
  println!("Day 1: Test 1: depth increases {} times", test_out);

  let part1_out = part1(&data);
  assert_eq!(part1_out, 1226);
  println!("Day 1: Part 1: depth increases {} times", part1_out);

  let test2_out = part2(&test);
  assert_eq!(test2_out, 5);
  println!("Day 1: Test 2: depth increases {} times", test2_out);

  let part2_out = part2(&data);
  assert_eq!(part2_out, 1252);
  println!("Day 1: Part 2: depth increases {} times", part2_out);
}

fn part1(depths: &[i32]) -> i32 {
  let mut increases = 0;

  for index in 1..depths.len() {
    if depths[index] > depths[index - 1] {
      increases += 1;
    }
  }

  increases
}

fn part2(depths: &[i32]) -> i32 {
  let mut increases = 0;

  for index in 3..depths.len() {
    if depths[index] > depths[index - 3] {
      increases += 1;
    }
  }

  increases
}
