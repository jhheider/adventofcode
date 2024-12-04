use std::cmp::max;

use crate::data;

pub fn main() {
  let data = data::Data::get(11);
  let test = data
    .test
    .lines()
    .map(|line| {
      line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>()
    })
    .collect::<Vec<Vec<i32>>>();
  let input = data
    .input
    .lines()
    .map(|line| {
      line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>()
    })
    .collect::<Vec<Vec<i32>>>();

  let test1 = flash(&test, 100);
  assert_eq!(test1, 1656);
  println!("Day 11: Test 1: {} flashes after 100 steps", test1);

  let part1 = flash(&input, 100);
  println!("Day 11: Part 1: {} flashes after 100 steps", part1);

  let test2 = flash(&test, u32::MAX);
  assert_eq!(test2, 195);
  println!("Day 11: Test 2: synchronizes after {} steps", test2);

  let part2 = flash(&input, u32::MAX);
  println!("Day 11: Part 2: synchronizes after {} steps", part2);
}

fn flash(input: &[Vec<i32>], steps: u32) -> u32 {
  let mut state = input.to_owned();
  let mut flashes = 0;

  for step in 0..steps {
    for row in state.iter_mut() {
      for cell in row.iter_mut() {
        *cell += 1;
      }
    }

    let mut flashed = true;
    while flashed {
      flashed = false;
      for y in 0..10 {
        for x in 0..10 {
          if state[y][x] > 9 {
            state[y][x] = i32::MIN;
            flashes += 1;
            flashed = true;
            for dy in -1..=1 {
              for dx in -1..=1 {
                if ((y as i32 + dy) >= 0)
                  && ((y as i32 + dy) <= 9)
                  && ((x as i32 + dx) >= 0)
                  && ((x as i32 + dx) <= 9)
                {
                  state[(y as i32 + dy) as usize][(x as i32 + dx) as usize] += 1;
                }
              }
            }
          }
        }
      }
    }
    state = state
      .iter()
      .map(|line| line.iter().map(|cell| max(*cell, 0)).collect())
      .collect();

    if state
      .iter()
      .map(|line| line.iter().sum::<i32>())
      .sum::<i32>()
      == 0
    {
      return step + 1;
    }
  }
  flashes
}
