use std::{
  fmt::{Display, Error, Formatter},
  fs,
};

const TEST: &str = r"30373
25512
65332
33549
35390";

struct Forest {
  size: usize,
  trees: Vec<Vec<Tree>>,
}

impl Forest {
  fn new(input: &str) -> Self {
    let mut trees = Vec::new();
    for line in input.lines() {
      let mut row = Vec::new();
      for c in line.chars() {
        row.push(Tree::new(c));
      }
      trees.push(row);
    }
    let mut forest = Forest {
      size: trees.len(),
      trees,
    };
    forest.mark_visibility();
    forest.set_score();
    forest
  }

  fn mark_visibility(&mut self) {
    for i in 0..self.size {
      for j in 0..self.size {
        if i == 0 || j == 0 || i == self.size - 1 || j == self.size - 1 {
          self.trees[i][j].visible = true;
          continue;
        }
        let up = self.trees[0..i]
          .iter()
          .map(|row| row[j].height)
          .max()
          .unwrap();
        let down = self.trees[i + 1..self.size]
          .iter()
          .map(|row| row[j].height)
          .max()
          .unwrap();
        let left = self.trees[i][0..j]
          .iter()
          .map(|tree| tree.height)
          .max()
          .unwrap();
        let right = self.trees[i][j + 1..self.size]
          .iter()
          .map(|tree| tree.height)
          .max()
          .unwrap();
        let min = left.min(right).min(up).min(down);
        if self.trees[i][j].height > min {
          self.trees[i][j].visible = true;
        }
      }
    }
  }

  fn count_visible(&self) -> usize {
    self
      .trees
      .iter()
      .map(|row| row.iter().filter(|t| t.visible).count())
      .sum()
  }

  fn set_score(&mut self) {
    for i in 1..self.size - 1 {
      for j in 1..self.size - 1 {
        let up = {
          let mut up = 0;
          for k in (0..i).rev() {
            up += 1;
            if self.trees[k][j].height >= self.trees[i][j].height {
              break;
            }
          }
          up
        };
        let down = {
          let mut down = 0;
          for k in i + 1..self.size {
            down += 1;
            if self.trees[k][j].height >= self.trees[i][j].height {
              break;
            }
          }
          down
        };
        let left = {
          let mut left = 0;
          for k in (0..j).rev() {
            left += 1;
            if self.trees[i][k].height >= self.trees[i][j].height {
              break;
            }
          }
          left
        };
        let right = {
          let mut right = 0;
          for k in j + 1..self.size {
            right += 1;
            if self.trees[i][k].height >= self.trees[i][j].height {
              break;
            }
          }
          right
        };
        self.trees[i][j].score = up * down * left * right;
      }
    }
  }

  fn best_score(&self) -> usize {
    self
      .trees
      .iter()
      .map(|row| row.iter().map(|t| t.score).max().unwrap())
      .max()
      .unwrap()
  }
}

impl Display for Forest {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    for row in &self.trees {
      for tree in row {
        write!(f, "{}", tree)?;
      }
      writeln!(f)?;
    }
    Ok(())
  }
}

struct Tree {
  height: usize,
  visible: bool,
  score: usize,
}

impl Tree {
  fn new(c: char) -> Self {
    let height = c.to_digit(10).unwrap() as usize;
    Tree {
      height,
      visible: false,
      score: 0,
    }
  }
}

impl Display for Tree {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    if self.visible {
      write!(f, "X")
    } else {
      write!(f, " ")
    }
  }
}

pub fn main() {
  let test = Forest::new(TEST);
  let test1 = test.count_visible();
  assert_eq!(test1, 21);
  println!("Day 8: Test 1: {}", test1);

  let input = Forest::new(&fs::read_to_string("data/day8.txt").unwrap());
  let part1 = input.count_visible();
  assert_eq!(part1, 1805);
  println!("Day 8: Part 1: {}", part1);

  let test2 = test.best_score();
  assert_eq!(test2, 8);
  println!("Day 8: Test 2: {}", test2);

  let part2 = input.best_score();
  assert_eq!(part2, 444528);
  println!("Day 8: Part 2: {}", part2);
}
