use std::fmt::{Display, Error, Formatter};
use std::fs;

const TEST: &str = "ULL
RRDDD
LURDL
UUUUD";

trait Button: Clone + Display {
  fn next(&self, dir: Direction) -> Self;
}

#[derive(Clone)]
enum NineKeyButton {
  One,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
}

#[derive(Clone)]
enum SpecialButton {
  One,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  A,
  B,
  C,
  D,
}

enum Direction {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Clone)]
struct Code<T: Button> {
  buttons: Vec<T>,
  pos: T,
}

impl From<char> for Direction {
  fn from(c: char) -> Self {
    match c {
      'U' => Direction::Up,
      'D' => Direction::Down,
      'L' => Direction::Left,
      'R' => Direction::Right,
      _ => panic!("Invalid direction"),
    }
  }
}

impl From<NineKeyButton> for char {
  fn from(b: NineKeyButton) -> char {
    match b {
      NineKeyButton::One => '1',
      NineKeyButton::Two => '2',
      NineKeyButton::Three => '3',
      NineKeyButton::Four => '4',
      NineKeyButton::Five => '5',
      NineKeyButton::Six => '6',
      NineKeyButton::Seven => '7',
      NineKeyButton::Eight => '8',
      NineKeyButton::Nine => '9',
    }
  }
}

impl From<SpecialButton> for char {
  fn from(b: SpecialButton) -> char {
    match b {
      SpecialButton::One => '1',
      SpecialButton::Two => '2',
      SpecialButton::Three => '3',
      SpecialButton::Four => '4',
      SpecialButton::Five => '5',
      SpecialButton::Six => '6',
      SpecialButton::Seven => '7',
      SpecialButton::Eight => '8',
      SpecialButton::Nine => '9',
      SpecialButton::A => 'A',
      SpecialButton::B => 'B',
      SpecialButton::C => 'C',
      SpecialButton::D => 'D',
    }
  }
}

impl Display for NineKeyButton {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    write!(f, "{}", Into::<char>::into(self.clone()))
  }
}

impl Display for SpecialButton {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    write!(f, "{}", Into::<char>::into(self.clone()))
  }
}

impl<T> Display for Code<T>
where
  T: Button,
{
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    let mut s = String::new();
    for b in &self.buttons {
      s.push_str(&b.to_string());
    }
    write!(f, "{}", s)
  }
}

impl Button for NineKeyButton {
  fn next(&self, dir: Direction) -> NineKeyButton {
    match dir {
      Direction::Up => match self {
        NineKeyButton::One => NineKeyButton::One,
        NineKeyButton::Two => NineKeyButton::Two,
        NineKeyButton::Three => NineKeyButton::Three,
        NineKeyButton::Four => NineKeyButton::One,
        NineKeyButton::Five => NineKeyButton::Two,
        NineKeyButton::Six => NineKeyButton::Three,
        NineKeyButton::Seven => NineKeyButton::Four,
        NineKeyButton::Eight => NineKeyButton::Five,
        NineKeyButton::Nine => NineKeyButton::Six,
      },
      Direction::Down => match self {
        NineKeyButton::One => NineKeyButton::Four,
        NineKeyButton::Two => NineKeyButton::Five,
        NineKeyButton::Three => NineKeyButton::Six,
        NineKeyButton::Four => NineKeyButton::Seven,
        NineKeyButton::Five => NineKeyButton::Eight,
        NineKeyButton::Six => NineKeyButton::Nine,
        NineKeyButton::Seven => NineKeyButton::Seven,
        NineKeyButton::Eight => NineKeyButton::Eight,
        NineKeyButton::Nine => NineKeyButton::Nine,
      },
      Direction::Left => match self {
        NineKeyButton::One => NineKeyButton::One,
        NineKeyButton::Two => NineKeyButton::One,
        NineKeyButton::Three => NineKeyButton::Two,
        NineKeyButton::Four => NineKeyButton::Four,
        NineKeyButton::Five => NineKeyButton::Four,
        NineKeyButton::Six => NineKeyButton::Five,
        NineKeyButton::Seven => NineKeyButton::Seven,
        NineKeyButton::Eight => NineKeyButton::Seven,
        NineKeyButton::Nine => NineKeyButton::Eight,
      },
      Direction::Right => match self {
        NineKeyButton::One => NineKeyButton::Two,
        NineKeyButton::Two => NineKeyButton::Three,
        NineKeyButton::Three => NineKeyButton::Three,
        NineKeyButton::Four => NineKeyButton::Five,
        NineKeyButton::Five => NineKeyButton::Six,
        NineKeyButton::Six => NineKeyButton::Six,
        NineKeyButton::Seven => NineKeyButton::Eight,
        NineKeyButton::Eight => NineKeyButton::Nine,
        NineKeyButton::Nine => NineKeyButton::Nine,
      },
    }
  }
}

impl Button for SpecialButton {
  fn next(&self, dir: Direction) -> SpecialButton {
    match dir {
      Direction::Up => match self {
        SpecialButton::One => SpecialButton::One,
        SpecialButton::Two => SpecialButton::Two,
        SpecialButton::Three => SpecialButton::One,
        SpecialButton::Four => SpecialButton::Four,
        SpecialButton::Five => SpecialButton::Five,
        SpecialButton::Six => SpecialButton::Two,
        SpecialButton::Seven => SpecialButton::Three,
        SpecialButton::Eight => SpecialButton::Four,
        SpecialButton::Nine => SpecialButton::Nine,
        SpecialButton::A => SpecialButton::Six,
        SpecialButton::B => SpecialButton::Seven,
        SpecialButton::C => SpecialButton::Eight,
        SpecialButton::D => SpecialButton::B,
      },
      Direction::Down => match self {
        SpecialButton::One => SpecialButton::Three,
        SpecialButton::Two => SpecialButton::Six,
        SpecialButton::Three => SpecialButton::Seven,
        SpecialButton::Four => SpecialButton::Eight,
        SpecialButton::Five => SpecialButton::Five,
        SpecialButton::Six => SpecialButton::A,
        SpecialButton::Seven => SpecialButton::B,
        SpecialButton::Eight => SpecialButton::C,
        SpecialButton::Nine => SpecialButton::Nine,
        SpecialButton::A => SpecialButton::A,
        SpecialButton::B => SpecialButton::D,
        SpecialButton::C => SpecialButton::C,
        SpecialButton::D => SpecialButton::D,
      },
      Direction::Left => match self {
        SpecialButton::One => SpecialButton::One,
        SpecialButton::Two => SpecialButton::Two,
        SpecialButton::Three => SpecialButton::Two,
        SpecialButton::Four => SpecialButton::Three,
        SpecialButton::Five => SpecialButton::Five,
        SpecialButton::Six => SpecialButton::Five,
        SpecialButton::Seven => SpecialButton::Six,
        SpecialButton::Eight => SpecialButton::Seven,
        SpecialButton::Nine => SpecialButton::Eight,
        SpecialButton::A => SpecialButton::A,
        SpecialButton::B => SpecialButton::A,
        SpecialButton::C => SpecialButton::B,
        SpecialButton::D => SpecialButton::D,
      },
      Direction::Right => match self {
        SpecialButton::One => SpecialButton::One,
        SpecialButton::Two => SpecialButton::Three,
        SpecialButton::Three => SpecialButton::Four,
        SpecialButton::Four => SpecialButton::Four,
        SpecialButton::Five => SpecialButton::Six,
        SpecialButton::Six => SpecialButton::Seven,
        SpecialButton::Seven => SpecialButton::Eight,
        SpecialButton::Eight => SpecialButton::Nine,
        SpecialButton::Nine => SpecialButton::Nine,
        SpecialButton::A => SpecialButton::B,
        SpecialButton::B => SpecialButton::C,
        SpecialButton::C => SpecialButton::C,
        SpecialButton::D => SpecialButton::D,
      },
    }
  }
}

impl<T> Code<T>
where
  T: Button,
{
  fn apply(&mut self, code: String) -> &mut Code<T> {
    for line in code.lines() {
      for c in line.chars() {
        let dir: Direction = c.into();
        self.pos = self.pos.next(dir);
      }
      self.buttons.push(self.pos.clone());
    }
    self
  }
}

pub fn main() {
  let start = Code {
    buttons: Vec::new(),
    pos: NineKeyButton::Five,
  };
  let input = fs::read_to_string("data/day2.txt").unwrap();

  let mut test1 = start.clone();
  test1.apply(TEST.to_string());
  println!("Day 2, Test 1: {}", test1);

  let mut p1 = start.clone();
  p1.apply(input.clone());
  println!("Day 2, Part 1: {}", p1);

  let start2 = Code {
    buttons: Vec::new(),
    pos: SpecialButton::Five,
  };

  let mut test2 = start2.clone();
  test2.apply(TEST.to_string());
  println!("Day 2, Test 2: {}", test2);

  let mut p2 = start2.clone();
  p2.apply(input.clone());
  println!("Day 2, Part 2: {}", p2);
}
