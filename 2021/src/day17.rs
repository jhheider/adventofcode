struct Position {
  x: isize,
  y: isize,
}

#[derive(Debug)]
struct Velocity {
  x: isize,
  y: isize,
}

#[derive(Debug)]
struct Area {
  x1: isize,
  y1: isize,
  x2: isize,
  y2: isize,
}

impl Area {
  fn contains(&self, p: &Position) -> bool {
    self.x1 <= p.x && self.y1 <= p.y && self.x2 >= p.x && self.y2 >= p.y
  }
  fn overshot(&self, p: &Position) -> bool {
    self.x2 < p.x || self.y1 > p.y
  }
}

pub fn main() {
  let test = Area {
    x1: 20,
    y1: -10,
    x2: 30,
    y2: -5,
  };
  let input = Area {
    x1: 117,
    y1: -140,
    x2: 164,
    y2: -89,
  };

  let test1 = part1(&test).0;
  assert_eq!(test1, 45);
  println!("Day 17: Test 1: {test1}");
  let p1 = part1(&input).0;
  assert_eq!(p1, 9730);
  println!("Day 17: Part 1: {p1}");

  let test2 = part1(&test).1;
  assert_eq!(test2, 112);
  println!("Day 17: Test 2: {test2}");
  let p2 = part1(&input).1;
  assert_eq!(p2, 4110);
  println!("Day 17: Part 2: {p2}");
}

fn sum(x: isize) -> isize {
  x * (x + 1) / 2
}

fn position(v: &Velocity, t: isize) -> Position {
  let x = if t > v.x {
    sum(v.x)
  } else {
    sum(v.x) - sum(v.x - t)
  };
  let y = t * v.y - sum(t - 1);
  Position { x, y }
}

fn run(v: Velocity, a: &Area) -> (isize, bool) {
  let mut ymax = isize::MIN;
  for t in 1.. {
    let p = position(&v, t);
    if p.y > ymax {
      ymax = p.y
    }
    if a.contains(&p) {
      return (ymax, true);
    }
    if a.overshot(&p) {
      return (ymax, false);
    }
  }
  (0, false)
}

fn part1(a: &Area) -> (isize, isize) {
  let mut ymax = 0;
  let mut count = 0;
  for x in 0..=a.x2 {
    for y in a.y1..a.x2 {
      let (ytot, hits) = run(Velocity { x, y }, a);
      if hits {
        count += 1;
        if ytot > ymax {
          ymax = ytot;
        }
      }
    }
  }
  (ymax, count)
}
