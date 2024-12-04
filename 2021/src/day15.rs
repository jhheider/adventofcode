use std::{cmp::min, collections::HashSet};

use crate::data::Data;

pub fn main() {
  let data = Data::get(15);
  let test = data
    .test
    .lines()
    .map(|line| {
      line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>()
    })
    .collect::<Vec<Vec<usize>>>();
  let input = data
    .input
    .lines()
    .map(|line| {
      line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>()
    })
    .collect::<Vec<Vec<usize>>>();

  let test1 = run(&test, false);
  assert_eq!(test1, 40);
  println!("Day 15: Test 1: safest path is {}", test1);

  let part1 = run(&input, false);
  println!("Day 15: Part 1: safest path is {}", part1);

  let test2 = run(&test, true);
  assert_eq!(test2, 315);
  println!("Day 15: Test 2: safest path is {}", test2);

  let part2 = run(&input, true);
  println!("Day 15: Test 2: safest path is {}", part2);
}

fn run(input: &[Vec<usize>], tiled: bool) -> usize {
  let end = match tiled {
    false => input.len() - 1,
    true => input.len() * 5 - 1,
  };
  let mut checked = HashSet::new();
  let mut queue = HashSet::new();

  let mut nodes = (0..=end)
    .map(|x| {
      (0..=end)
        .map(|y| {
          if (x, y) == (0, 0) {
            return 0;
          }
          usize::MAX
        })
        .collect::<Vec<usize>>()
    })
    .collect::<Vec<Vec<usize>>>();

  let mut current = (0, 0);
  while current != (end, end) {
    queue.remove(&current);
    for d in [-1, 1] {
      for neighbor in [
        (current.0 as i32 + d, current.1 as i32),
        (current.0 as i32, current.1 as i32 + d),
      ] {
        if (neighbor.0 >= 0)
          && (neighbor.0 <= end as i32)
          && (neighbor.1 >= 0)
          && (neighbor.1 <= end as i32)
        {
          let neighbor = (neighbor.0 as usize, neighbor.1 as usize);
          if !checked.contains(&neighbor) {
            nodes[neighbor.0][neighbor.1] = min(
              nodes[neighbor.0][neighbor.1],
              nodes[current.0][current.1] + risk(input, neighbor),
            );
            queue.insert(neighbor);
          }
        }
      }
    }

    checked.insert(current);
    current = queue.iter().fold((end, end), |next, node| {
      if nodes[node.0][node.1] < nodes[next.0][next.1] {
        return *node;
      }
      next
    });
    // println!("{:?}", current);
  }
  // println!("{:?}", nodes);
  nodes[end][end]
}

fn risk(input: &[Vec<usize>], node: (usize, usize)) -> usize {
  let size = input.len();
  (input[node.0 % size][node.1 % size] + node.0 / size + node.1 / size - 1) % 9 + 1
}
