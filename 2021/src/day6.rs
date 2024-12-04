use crate::data::Data;

pub fn main() {
  let data = Data::get(6);
  let test = parse(&data.test);
  let input = parse(&data.input);

  let test1a = run(&test, 18);
  assert_eq!(test1a, 26);
  println!("Day 6: Test 1a: after 18 days, there are {} fish", test1a);

  let test1b = run(&test, 80);
  assert_eq!(test1b, 5934);
  println!("Day 6: Test 1a: after 80 days, there are {} fish", test1b);

  let part1 = run(&input, 80);
  println!("Day 6: Part 1: after 80 days, there are {} fish", part1);

  let test2 = run(&test, 256);
  assert_eq!(test2, 26984457539);
  println!("Day 6: Test 2: after 256 days, there are {} fish", test2);

  let part2 = run(&input, 256);
  println!("Day 6: Part 2: after 256 days, there are {} fish", part2);
}

fn parse(input: &str) -> Vec<u64> {
  input
    .split(',')
    .map(|s| s.parse::<u64>().unwrap())
    .collect::<Vec<u64>>()
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
