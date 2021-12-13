use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Edge {
  a: Cave,
  b: Cave,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Cave {
  Start,
  End,
  Small(String),
  Large(String),
}

impl Clone for Cave {
  fn clone(&self) -> Self {
    match self {
      Cave::Start => Cave::Start,
      Cave::End => Cave::End,
      Cave::Small(s) => Cave::Small(s.clone()),
      Cave::Large(s) => Cave::Large(s.clone()),
    }
  }
}

impl Cave {
  fn from(value: String) -> Self {
    match value.as_str() {
      "start" => Cave::Start,
      "end" => Cave::End,
      v if v == v.to_uppercase() => Cave::Large(v.to_string()),
      _ => Cave::Small(value),
    }
  }
}

pub fn main() {
  let tests = vec![
    parse(
      r"start-A
start-b
A-c
A-b
b-d
A-end
b-end",
    ),
    parse(
      r"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc",
    ),
    parse(
      r"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW",
    ),
  ];
  let input_string = fs::read_to_string("data/day12.txt").unwrap();
  let input = parse(&input_string);

  let test1 = tests
    .iter()
    .map(|test| routes(test, false))
    .collect::<Vec<i32>>();
  assert_eq!(test1, [10, 19, 226]);
  println!("Day 12: Test 1: {} routes", test1[0]);
  println!("Day 12: Test 2: {} routes", test1[1]);
  println!("Day 12: Test 3: {} routes", test1[2]);

  let part1 = routes(&input, false);
  assert_eq!(part1, 4549);
  println!("Day 12: Part 1: {} routes", part1);

  let test2 = tests
    .iter()
    .map(|test| routes(test, true))
    .collect::<Vec<i32>>();
  assert_eq!(test2, [36, 103, 3509]);
  println!("Day 12: Test 4: {} routes", test2[0]);
  println!("Day 12: Test 5: {} routes", test2[1]);
  println!("Day 12: Test 6: {} routes", test2[2]);

  let part2 = routes(&input, true);
  assert_eq!(part2, 120535);
  println!("Day 12: Part 2: {} routes", part2);
}

fn parse(input: &str) -> Vec<Edge> {
  input
    .lines()
    .map(|line| {
      let mut caves = line.split('-');

      let a = Cave::from(caves.next().unwrap().to_string());
      let b = Cave::from(caves.next().unwrap().to_string());
      Edge { a, b }
    })
    .collect()
}

fn routes(input: &[Edge], part2: bool) -> i32 {
  let mut routes = vec![(vec![Cave::Start], HashMap::<Cave, i32>::new())];
  let mut count = 0;

  while !routes.is_empty() {
    let (head, seen) = routes.remove(0);
    let last = head.last().unwrap().clone();

    for hop in input.iter() {
      for (start, end) in [(&hop.a, &hop.b), (&hop.b, &hop.a)] {
        if *start == last {
          match end {
            Cave::End => count += 1,
            Cave::Start => continue,
            Cave::Large(_) => {
              let mut next = head.clone();
              next.push(end.clone());
              routes.push((next, seen.clone()));
            }
            Cave::Small(_) => {
              if head.contains(end) && (!part2 || seen.values().any(|x| *x == 2)) {
                continue;
              }
              let mut next = head.clone();
              next.push(end.clone());
              let mut new_seen = seen.clone();
              new_seen.insert(end.clone(), seen.get(end).unwrap_or(&0) + 1);
              routes.push((next, new_seen));
            }
          }
        }
      }
    }
  }
  count
}
