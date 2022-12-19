use std::{
  collections::HashSet,
  fmt::{Display, Formatter},
};

const TEST: &str = r">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

enum RockType {
  Long,
  Plus,
  Angle,
  Tall,
  Box,
}

struct Rock {
  position: Vec<(usize, usize)>,
}

#[derive(Copy, Clone)]
enum Direction {
  Left,
  Right,
}

struct Chamber {
  height: usize,
  rocks: HashSet<(usize, usize)>,
  jets: Vec<Direction>,
  time: usize,
  next_rock: usize,
}

#[derive(PartialEq, Hash, Eq, Debug)]
struct Cache {
  heights: [isize; 7],
  next_rock: usize,
  jets_index: usize,
}

impl RockType {
  fn index(i: usize) -> Self {
    match i % 5 {
      0 => RockType::Long,
      1 => RockType::Plus,
      2 => RockType::Angle,
      3 => RockType::Tall,
      4 => RockType::Box,
      _ => unreachable!(),
    }
  }
}

impl Rock {
  fn new(height: usize, time: usize) -> Self {
    match RockType::index(time) {
      RockType::Long => Rock {
        position: (2..6).map(|i| (i, height + 4)).collect(),
      },
      RockType::Plus => Rock {
        position: vec![
          (2, height + 5),
          (3, height + 5),
          (4, height + 5),
          (3, height + 4),
          (3, height + 6),
        ],
      },
      RockType::Angle => Rock {
        position: vec![
          (2, height + 4),
          (3, height + 4),
          (4, height + 4),
          (4, height + 5),
          (4, height + 6),
        ],
      },
      RockType::Tall => Rock {
        position: (0..4).map(|i| (2, height + i + 4)).collect(),
      },
      RockType::Box => Rock {
        position: vec![
          (2, height + 4),
          (3, height + 4),
          (2, height + 5),
          (3, height + 5),
        ],
      },
    }
  }
}

impl Direction {
  fn offset(&self) -> isize {
    match self {
      Direction::Left => -1,
      Direction::Right => 1,
    }
  }
}

impl Chamber {
  fn new(input: &str) -> Self {
    let rocks = HashSet::new();
    let jets = input
      .chars()
      .map(|c| match c {
        '>' => Direction::Right,
        '<' => Direction::Left,
        _ => unreachable!(),
      })
      .collect();
    Chamber {
      height: 0,
      rocks,
      jets,
      time: 0,
      next_rock: 0,
    }
  }

  fn add_rock(&mut self) {
    let mut rock = Rock::new(self.height, self.next_rock);
    self.next_rock += 1;
    loop {
      let offset = self.jets[self.time % self.jets.len()].offset();
      if rock.position.iter().all(|(x, y)| {
        *x as isize + offset >= 0
          && *x as isize + offset <= 6
          && !self.rocks.contains(&((*x as isize + offset) as usize, *y))
      }) {
        rock.position = rock
          .position
          .iter()
          .map(|(x, y)| ((*x as isize + offset) as usize, *y))
          .collect();
      } else {
        rock.position = rock.position.iter().map(|(x, y)| (*x, *y)).collect();
      }
      self.time += 1;
      if rock
        .position
        .iter()
        .any(|(x, y)| *y == 1 || self.rocks.contains(&(*x, *y - 1)))
      {
        self.rocks.extend(rock.position);
        self.height = self.rocks.iter().map(|(_, y)| *y).max().unwrap();
        break;
      } else {
        rock.position = rock.position.iter().map(|(x, y)| (*x, *y - 1)).collect();
      }
    }
  }

  fn add_many(&mut self, n: usize) -> usize {
    let mut cache = HashSet::new();
    for i in 0..n {
      if i % 1_000_000 == 0 {
        println!("{}", i);
      }
      self.add_rock();
      let new_cache = self.make_cache();
      if cache.contains(&new_cache) {
        println!("Found a cycle!, {}", i);
        break;
      }
      cache.insert(new_cache);
    }
    self.height
  }

  fn make_cache(&mut self) -> Cache {
    let mut depth = [self.height as isize; 7];
    self.rocks.retain(|(x, y)| {
      depth[*x] = depth[*x].min(self.height as isize - *y as isize);
      self.height - y < 100
    });
    Cache {
      heights: depth,
      next_rock: self.next_rock,
      jets_index: self.time % self.jets.len(),
    }
  }
}

impl Display for Chamber {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    for y in (0..=self.height + 3).rev() {
      if y == 0 {
        writeln!(f, "+-------+")?;
        continue;
      }
      write!(f, "|")?;
      for x in 0..=6 {
        if self.rocks.contains(&(x, y)) {
          write!(f, "#")?;
        } else {
          write!(f, ".")?;
        }
      }
      writeln!(f, "|")?;
    }
    Ok(())
  }
}
pub fn main() {
  let mut test = Chamber::new(TEST);
  let test1 = test.add_many(2022);
  assert_eq!(test1, 3068);
  println!("Day 17: Test 1: {}", test1);

  let mut input = Chamber::new(include_str!("../../data/day17.txt"));
  let part1 = input.add_many(2022);
  assert_eq!(part1, 3065);
  println!("Day 17: Part 1: {}", part1);

  let mut test = Chamber::new(TEST);
  let test2 = test.add_many(1_000_000_000_000);
  assert_eq!(test2, 1_514_285_714_288);
  println!("Day 17: Test 2: {}", test2);
}
