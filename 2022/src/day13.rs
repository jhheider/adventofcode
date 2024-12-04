use std::cmp::Ordering;

use crate::data::Data;

#[derive(Debug)]
struct Pair {
  left: Packet,
  right: Packet,
}

#[derive(Debug, Clone)]
enum Packet {
  List(Vec<Packet>),
  Number(usize),
}

impl Pair {
  fn parse(input: String) -> Self {
    let mut split = input.split('\n');
    let left = Packet::parse(split.next().unwrap().to_string()).0;
    let right = Packet::parse(split.next().unwrap().to_string()).0;
    Self { left, right }
  }

  fn in_order(&self) -> bool {
    self.left.compare(&self.right) != Ordering::Greater
  }
}

impl Packet {
  fn parse(input: String) -> (Self, String) {
    if input.starts_with('[') {
      let mut list = Vec::new();
      let mut input = input.strip_prefix('[').unwrap().to_string();
      while !input.starts_with(']') {
        let (packet, rest) = Self::parse(input);
        list.push(packet);
        input = rest.trim_start_matches(',').to_string();
      }
      (Self::List(list), input[1..].to_string())
    } else {
      let number = input
        .split([',', ']'])
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();
      let rest = input[number.to_string().len()..].to_string();
      (Self::Number(number), rest)
    }
  }

  fn compare(&self, right: &Packet) -> Ordering {
    match (self, right) {
      (Self::Number(left), Self::Number(right)) if left < right => Ordering::Less,
      (Self::Number(left), Self::Number(right)) if left == right => Ordering::Equal,
      (Self::Number(_), Self::Number(_)) => Ordering::Greater,
      (Self::List(left), Self::List(right)) => {
        let mut left = left.iter();
        let mut right = right.iter();
        loop {
          match (left.next(), right.next()) {
            (Some(left), Some(right)) => {
              let result = left.compare(right);
              if result != Ordering::Equal {
                return result;
              }
            }
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            (None, None) => return Ordering::Equal,
          }
        }
      }
      (Self::Number(_), Self::List(_)) => Packet::List(vec![self.clone()]).compare(right),
      (Self::List(_), Self::Number(_)) => self.compare(&Packet::List(vec![right.clone()])),
    }
  }
}

pub fn main() {
  let data = Data::get(13);
  let mut test = data
    .test
    .split("\n\n")
    .map(|s| Pair::parse(s.to_string()))
    .collect::<Vec<_>>();
  let test1 = sum_packet_passes(&test);
  assert_eq!(test1, 13);
  println!("Day 13: Test 1: {}", test1);

  let mut input = data
    .input
    .split("\n\n")
    .map(|s| Pair::parse(s.to_string()))
    .collect::<Vec<_>>();
  let part1 = sum_packet_passes(&input);
  println!("Day 13: Part 1: {}", part1);

  test.push(Pair {
    left: Packet::List(vec![Packet::List(vec![Packet::Number(2)])]),
    right: Packet::List(vec![Packet::List(vec![Packet::Number(6)])]),
  });
  let test2 = decoder_key(&test);
  assert_eq!(test2, 140);
  println!("Day 13: Test 2: {}", test2);

  input.push(Pair {
    left: Packet::List(vec![Packet::List(vec![Packet::Number(2)])]),
    right: Packet::List(vec![Packet::List(vec![Packet::Number(6)])]),
  });
  let part2 = decoder_key(&input);
  println!("Day 13: Part 2: {}", part2);
}

fn sum_packet_passes(input: &[Pair]) -> usize {
  input
    .iter()
    .enumerate()
    .filter_map(|(i, p)| match !p.in_order() {
      true => None,
      false => Some(i + 1),
    })
    .sum::<usize>()
}

fn sort_packets(input: &[Pair]) -> Vec<Packet> {
  let mut input = input
    .iter()
    .flat_map(|p| vec![p.left.clone(), p.right.clone()])
    .collect::<Vec<_>>();
  input.sort_by(|a, b| a.compare(b));

  input
}

fn decoder_key(input: &[Pair]) -> usize {
  sort_packets(input)
    .iter()
    .enumerate()
    .filter_map(|(i, p)| match p {
      Packet::List(p1) => {
        if p1.len() != 1 {
          return None;
        }
        match p1.first() {
          Some(Packet::List(p2)) => {
            if p2.len() != 1 {
              return None;
            }
            match p2.first() {
              Some(Packet::Number(n)) => {
                if *n == 2 || *n == 6 {
                  Some(i + 1)
                } else {
                  None
                }
              }
              _ => None,
            }
          }
          _ => None,
        }
      }
      _ => None,
    })
    .product::<usize>()
}
