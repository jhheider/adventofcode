use regex::Regex;

use crate::data::Data;

pub fn main() {
  let data = Data::get(4);
  let test1 = pt1(&data.test);
  assert_eq!(test1, 18);
  println!("Day 4: Test 1: {}", test1);

  let part1 = pt1(&data.input);
  println!("Day 4: Part 1: {}", part1);

  let test2 = pt2(&data.test);
  assert_eq!(test2, 9);
  println!("Day 4: Test 2: {}", test2);

  let part2 = pt2(&data.input);
  println!("Day 4: Part 2: {}", part2);
}

fn grid(input: &str) -> Vec<Vec<char>> {
  input.lines().map(|line| line.chars().collect()).collect()
}

fn transpose(matrix: &str) -> String {
  let matrix = grid(matrix);
  if matrix.is_empty() || matrix[0].is_empty() {
    return "".to_string();
  }

  let row_count = matrix.len();
  let col_count = matrix[0].len();

  let mut transposed = vec![Vec::with_capacity(row_count); col_count];

  for row in matrix {
    for (col_idx, val) in row.into_iter().enumerate() {
      transposed[col_idx].push(val);
    }
  }

  transposed
    .into_iter()
    .map(|line| line.into_iter().collect::<String>())
    .collect::<Vec<String>>()
    .join("\n")
}

fn pt1(input: &str) -> usize {
  let mut count = 0;

  // XMAS
  let regex_forward = Regex::new(r"XMAS").unwrap();
  count += regex_forward.find_iter(input).count();

  // SAMX
  let regex_backward = Regex::new(r"SAMX").unwrap();
  count += regex_backward.find_iter(input).count();

  // Vertical
  let flip: String = transpose(input);

  // Down
  count += regex_forward.find_iter(&flip).count();
  // Up
  count += regex_backward.find_iter(&flip).count();

  // Diagonals
  let grid = grid(input);
  for i in 0..grid.len() {
    for j in 0..grid[i].len() {
      if i + 3 < grid.len()
        && j + 3 < grid[i].len()
        && grid[i][j] == 'X'
        && grid[i + 1][j + 1] == 'M'
        && grid[i + 2][j + 2] == 'A'
        && grid[i + 3][j + 3] == 'S'
      {
        count += 1;
      }
      if i + 3 < grid.len()
        && j >= 3
        && grid[i][j] == 'X'
        && grid[i + 1][j - 1] == 'M'
        && grid[i + 2][j - 2] == 'A'
        && grid[i + 3][j - 3] == 'S'
      {
        count += 1;
      }
      if i >= 3
        && j + 3 < grid[i].len()
        && grid[i][j] == 'X'
        && grid[i - 1][j + 1] == 'M'
        && grid[i - 2][j + 2] == 'A'
        && grid[i - 3][j + 3] == 'S'
      {
        count += 1;
      }
      if i >= 3
        && j >= 3
        && grid[i][j] == 'X'
        && grid[i - 1][j - 1] == 'M'
        && grid[i - 2][j - 2] == 'A'
        && grid[i - 3][j - 3] == 'S'
      {
        count += 1;
      }
    }
  }
  count
}

fn pt2(input: &str) -> usize {
  let mut count = 0;
  let grid = grid(input);
  for i in 1..grid.len() - 1 {
    for j in 1..grid[i].len() - 1 {
      if grid[i][j] == 'A'
        && ((grid[i - 1][j - 1] == 'M' && grid[i + 1][j + 1] == 'S')
          || (grid[i - 1][j - 1] == 'S' && grid[i + 1][j + 1] == 'M'))
        && ((grid[i - 1][j + 1] == 'M' && grid[i + 1][j - 1] == 'S')
          || (grid[i - 1][j + 1] == 'S' && grid[i + 1][j - 1] == 'M'))
      {
        count += 1;
      }
    }
  }
  count
}
