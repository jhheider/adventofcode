use std::{collections::HashMap, fs};

pub fn main() {
  let test = parse_input(
    r"2199943210
3987894921
9856789892
8767896789
9899965678"
      .to_string(),
  );
  let input = parse_input(fs::read_to_string("data/day9.txt").unwrap());

  let test1 = find_risk(&test);
  assert_eq!(test1, 15);
  println!("Day 9: Test 1: the cumulative risk is {}", test1);

  let part1 = find_risk(&input);
  assert_eq!(part1, 462);
  println!("Day 9: Part 1: the cumulative risk is {}", part1);

  let test2 = find_basins(&test);
  assert_eq!(test2, 1134);
  println!("Day 9: Test 2: the basin product is {}", test2);

  let part2 = find_basins(&input);
  assert_eq!(part2, 1397760);
  println!("Day 9: Part 2: the basin product is {}", part2);
}

fn parse_input(input: String) -> Vec<Vec<u32>> {
  input
    .lines()
    .map(|line| {
      line
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>()
    })
    .collect::<Vec<Vec<u32>>>()
}

fn find_risk(input: &[Vec<u32>]) -> u32 {
  let mut risk = 0u32;

  for (y, row) in input.iter().enumerate() {
    for (x, cell) in row.iter().enumerate() {
      if (y > 0) && (cell >= &input[y - 1][x]) {
        continue;
      }
      if (x > 0) && (cell >= &input[y][x - 1]) {
        continue;
      }
      if (x < row.len() - 1) && (cell >= &input[y][x + 1]) {
        continue;
      }
      if (y < input.len() - 1) && (cell >= &input[y + 1][x]) {
        continue;
      }

      risk += 1 + cell;
    }
  }
  risk
}

fn find_basins(input: &[Vec<u32>]) -> i32 {
  let mut basins = input
    .iter()
    .enumerate()
    .flat_map(|(y, row)| {
      row
        .iter()
        .enumerate()
        .filter(|(_, cell)| **cell == 9)
        .map(move |(x, _)| ((y as isize, x as isize), -1))
    })
    .collect::<HashMap<(isize, isize), i32>>();
  let y_size = input.len() as isize;
  let x_size = input[0].len() as isize;

  for x in -1..=x_size {
    basins.insert((-1, x), -1);
    basins.insert((y_size, x), -1);
  }

  for y in -1..=y_size {
    basins.insert((y, -1), -1);
    basins.insert((y, x_size), -1);
  }

  let mut next_basin = 1;
  let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];

  for y in 0..y_size {
    for x in 0..x_size {
      match basins.get(&(y, x)) {
        Some(_) => continue,
        None => {
          let neighbors = offsets
            .iter()
            .map(|(dy, dx)| *basins.get(&(y + dy, x + dx)).unwrap_or(&0))
            .filter(|v| *v > 0)
            .collect::<Vec<i32>>();
          match neighbors.len() {
            0 => {
              basins.insert((y, x), next_basin);
              next_basin += 1;
            }
            1 => {
              basins.insert((y, x), neighbors[0]);
            }
            _ => {
              let dominant = neighbors.clone().iter().min().unwrap().to_owned();
              basins.insert((y, x), dominant);
              for v in neighbors.into_iter().filter(|v| *v != dominant) {
                for ((y, x), b) in basins.clone().iter() {
                  if *b == v {
                    basins.insert((*y, *x), dominant);
                  }
                }
              }
            }
          }
        }
      }
    }
  }

  let mut size_counts = HashMap::<i32, i32>::new();
  for b in basins.iter() {
    if b.1 == &-1 {
      continue;
    }
    size_counts.insert(*b.1, size_counts.get(b.1).unwrap_or(&0) + 1);
  }
  let mut sizes = size_counts
    .iter()
    .map(|v| v.1)
    .copied()
    .collect::<Vec<i32>>();
  sizes.sort_unstable();
  sizes.reverse();

  sizes[0] * sizes[1] * sizes[2]
}
