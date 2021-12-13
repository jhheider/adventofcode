use std::fs;

pub fn main() {
  let test = "3,4,3,1,2"
    .split(',')
    .map(|s| s.parse::<u64>().unwrap())
    .collect::<Vec<u64>>();
  let input = fs::read_to_string("data/day6.txt")
    .unwrap()
    .split(',')
    .map(|s| s.parse::<u64>().unwrap())
    .collect::<Vec<u64>>();

  let test1a = run(&test, 18);
  assert_eq!(test1a, 26);
  println!("Day 6: Test 1a: after 18 days, there are {} fish", test1a);

  let test1b = run(&test, 80);
  assert_eq!(test1b, 5934);
  println!("Day 6: Test 1a: after 80 days, there are {} fish", test1b);

  let part1 = run(&input, 80);
  assert_eq!(part1, 377263);
  println!("Day 6: Part 1: after 80 days, there are {} fish", part1);

  let test2 = run(&test, 256);
  assert_eq!(test2, 26984457539);
  println!("Day 6: Test 2: after 256 days, there are {} fish", test2);

  let part2 = run(&input, 256);
  assert_eq!(part2, 1695929023803);
  println!("Day 6: Part 2: after 256 days, there are {} fish", part2);
}

fn run(input: &[u64], days: u64) -> u64 {
  let mut state: Vec<u64> = vec![0; 9];

  for i in input {
    state[*i as usize] += 1;
  }

  for _ in 0..days {
    let mut next_state = vec![0; 9];

    for day in 0..=8 {
      match day {
        0 => {
          next_state[6] = state[0];
          next_state[8] = state[0];
        }
        _ => {
          next_state[day - 1] += state[day];
        }
      }
    }
    state = next_state
  }
  state.iter().sum()
}
