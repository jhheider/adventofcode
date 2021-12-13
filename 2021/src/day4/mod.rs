use std::{cmp::max, collections::HashSet, fs};

pub fn main() {
  let (test_moves, test_boards) = load("data/day4test.txt");
  let (input_moves, input_boards) = load("data/day4.txt");

  let test1 = run(&test_moves, &test_boards).unwrap();
  assert_eq!(test1.1, 4512);
  println!("Day 4: Test 1: Board #{} wins; score {}", test1.0, test1.1);

  let part1 = run(&input_moves, &input_boards).unwrap();
  assert_eq!(part1.1, 8580);
  println!("Day 4: Part 1: Board #{} wins; score {}", part1.0, part1.1);

  let test2 = run2(&test_moves, &test_boards).unwrap();
  assert_eq!(test2, 1924);
  println!("Day 4: Test 2: Score {}", test2);

  let part2 = run2(&input_moves, &input_boards).unwrap();
  assert_eq!(part2, 9576);
  println!("Day 4: Part 2: Score {}", part2);
}

fn load(file: &str) -> (Vec<i32>, Vec<Vec<Vec<i32>>>) {
  let input = fs::read_to_string(file).unwrap();
  let mut lines = input.lines();

  let moves = lines
    .next()
    .unwrap()
    .split(',')
    .map(|i| i.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();

  let mut boards = Vec::<Vec<Vec<i32>>>::new();

  while lines.next().is_some() {
    let mut board = Vec::<Vec<i32>>::new();

    for _ in 0..5 {
      board.push(
        lines
          .next()
          .unwrap()
          .split_whitespace()
          .map(|i| i.parse::<i32>().unwrap())
          .collect::<Vec<i32>>(),
      );
    }
    boards.push(board);
  }

  (moves, boards)
}

fn run(moves: &[i32], boards: &[Vec<Vec<i32>>]) -> Result<(i32, i32), ()> {
  let mut boards = boards.to_owned();

  for num in moves.iter() {
    for (board_num, board) in boards.iter_mut().enumerate() {
      for row in board.iter_mut() {
        for cell in row.iter_mut() {
          if cell == num {
            *cell = -1;
          }
        }
      }

      for x in 0..5 {
        if (board[x].iter().sum::<i32>() == -5)
          || (board.iter().map(|row| row[x]).sum::<i32>() == -5)
        {
          return Ok((
            board_num as i32 + 1,
            board
              .iter()
              .map(|row| row.iter().map(|i| max(*i, 0)).sum::<i32>())
              .sum::<i32>()
              * num,
          ));
        }
      }
    }
  }
  Err(())
}

fn run2(moves: &[i32], boards: &[Vec<Vec<i32>>]) -> Result<i32, ()> {
  let mut boards = boards.to_owned();

  for num in moves.iter() {
    let mut winners = HashSet::new();
    for (board_num, board) in boards.iter_mut().enumerate() {
      for row in board.iter_mut() {
        for cell in row.iter_mut() {
          if cell == num {
            *cell = -1;
          }
        }
      }

      for x in 0..5 {
        if (board[x].iter().sum::<i32>() == -5)
          || (board.iter().map(|row| row[x]).sum::<i32>() == -5)
        {
          winners.insert(board_num);
        }
      }
    }
    if (boards.len() == 1) && (winners.len() == 1) {
      let board = boards.first().unwrap();
      return Ok(
        board
          .iter()
          .map(|row| row.iter().map(|i| max(*i, 0)).sum::<i32>())
          .sum::<i32>()
          * num,
      );
    } else if boards.is_empty() {
      return Err(());
    } else {
      boards = boards
        .iter()
        .enumerate()
        .filter(|(idx, _)| !winners.contains(idx))
        .map(|(_, board)| board.clone())
        .collect::<Vec<Vec<Vec<i32>>>>();
      winners.clear();
    }
  }
  Err(())
}
