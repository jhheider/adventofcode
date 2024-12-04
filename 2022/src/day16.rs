use std::{
  collections::{HashMap, HashSet, VecDeque},
  rc::Rc,
};

use crate::data::Data;

#[derive(Clone, Debug)]
struct Valve {
  flow_rate: usize,
  tunnels: Vec<String>,
}

#[derive(Clone, Debug)]
struct Cave {
  valves: HashMap<String, Valve>,
}

#[derive(Clone, Debug)]
struct State {
  cave: Rc<Cave>,
  opened: HashSet<String>,
  pressure: usize,
  location: String,
  last: String,
  elephant_location: String,
  elephant_last: String,
  time: usize,
  history: Vec<(usize, String)>,
}

#[derive(PartialEq)]
enum Action {
  Open(String),
  Move(String),
}

impl Valve {
  fn new(line: String) -> Self {
    let mut parts = line.split(' ');
    let flow_rate = parts
      .nth(4)
      .unwrap()
      .split('=')
      .nth(1)
      .unwrap()
      .trim_end_matches(';')
      .parse()
      .unwrap();
    let tunnels = parts
      .skip(4)
      .map(|s| s.trim_end_matches(',').to_string())
      .collect();
    Self { flow_rate, tunnels }
  }
}

impl Cave {
  fn new(input: String) -> Self {
    let valves = input
      .lines()
      .map(|line| {
        (
          line.split(' ').nth(1).unwrap().to_string(),
          Valve::new(line.to_string()),
        )
      })
      .collect();
    Self { valves }
  }

  fn search(&self, with_elephant: bool) -> usize {
    let state = State {
      cave: Rc::new(self.clone()),
      opened: HashSet::new(),
      pressure: 0,
      location: "AA".to_string(),
      last: "".to_string(),
      elephant_location: if with_elephant {
        "AA".to_string()
      } else {
        "".to_string()
      },
      elephant_last: "".to_string(),
      time: if with_elephant { 4 } else { 0 },
      history: vec![],
    };
    state.search(with_elephant)
  }
}

impl State {
  fn open(&mut self, valve: String) {
    if self.opened.contains(&valve) {
      panic!("{} already opened", valve);
    }
    self.opened.insert(valve.clone());
    self.pressure += self.cave.valves.get(&valve).unwrap().flow_rate * (30 - self.time);
    self.history.push((self.time, valve));
  }

  fn search(&self, with_elephant: bool) -> usize {
    let mut iteration = 0;
    let mut queue = VecDeque::new();
    queue.push_back(self.clone());
    let mut best = usize::MIN;
    while let Some(mut state) = queue.pop_front() {
      state.time += 1;
      iteration += 1;
      if iteration % 10_000_000 == 0 {
        println!(
          "Iteration: {}: best: {}: queue length: {}",
          iteration,
          best,
          queue.len()
        );
      }
      best = best.max(state.pressure);
      if state.time == 30 {
        continue;
      }
      let available = state
        .cave
        .valves
        .iter()
        .filter_map(|(name, valve)| {
          if state.opened.contains(name) {
            return None;
          }
          Some(valve.flow_rate * (30 - state.time))
        })
        .sum::<usize>();
      if state.pressure + available < best {
        continue;
      }
      let mut next_moves = [vec![], vec![]];
      let current = state.cave.valves.get(&state.location).unwrap();
      for tunnel in current.tunnels.iter() {
        if *tunnel == state.last {
          continue;
        }
        next_moves[0].push(Action::Move(tunnel.clone()));
      }
      if !state.opened.contains(&state.location) && current.flow_rate > 0 {
        next_moves[0].push(Action::Open(state.location.clone()));
      }
      if with_elephant {
        let current2 = state.cave.valves.get(&state.elephant_location).unwrap();
        for tunnel in current2.tunnels.iter() {
          if *tunnel == state.elephant_last {
            continue;
          }
          next_moves[1].push(Action::Move(tunnel.clone()));
        }
        if !state.opened.contains(&state.elephant_location) && current2.flow_rate > 0 {
          next_moves[1].push(Action::Open(state.elephant_location.clone()));
        }
      }
      for action in next_moves[0].iter() {
        let mut next = state.clone();
        match action {
          Action::Open(valve) => {
            next.open(valve.clone());
            next.last = "".to_string();
          }
          Action::Move(tunnel) => {
            next.last = next.location;
            next.location = tunnel.clone();
          }
        }
        if with_elephant {
          for action2 in next_moves[1].iter() {
            let mut next2 = next.clone();
            match action2 {
              Action::Open(valve) => {
                if next2.opened.contains(valve) {
                  continue;
                }
                next2.open(valve.clone());
                next2.elephant_last = "".to_string();
              }
              Action::Move(tunnel) => {
                next2.elephant_last = next2.elephant_location;
                next2.elephant_location = tunnel.clone();
              }
            }
            queue.push_front(next2);
          }
        } else {
          queue.push_front(next);
        }
      }
    }
    best
  }
}

pub fn main() {
  println!("\n*** Warning: this one takes a while... ***\n");

  let data = Data::get(16);
  let test = Cave::new(data.test);
  let test1 = test.search(false);
  assert_eq!(test1, 1651);
  println!("Day 16: Test: 1: {}", test1);

  let input = Cave::new(data.input);
  let part1 = input.search(false);
  assert_eq!(part1, 1653);
  println!("Day 16: Part 1: {}", part1);

  let test2 = test.search(true);
  assert_eq!(test2, 1707);
  println!("Day 16: Test: 2: {}", test2);

  let part2 = input.search(true);
  assert_eq!(part2, 2223);
  println!("Day 16: Part 2: {}", part2);
}
