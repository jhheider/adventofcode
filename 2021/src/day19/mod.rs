use nalgebra::base::{Matrix3, Vector3};
use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Scanner {
  id: usize,
  beacons: Vec<Vector3<isize>>,
}

struct Beacon {}

pub fn main() {
  let scanners = parse(fs::read_to_string("data/test19.txt").unwrap());
  println!("{scanners:?}");
}

fn parse(input: String) -> Vec<Scanner> {
  let scans = input.split("\n\n");
  let mut scanners = Vec::new();

  for scan in scans {
    let mut beacons = Vec::new();
    let mut lines = scan.lines();
    let heading = lines.next().unwrap();
    let regex = Regex::new(r"--- scanner (\d+) ---").unwrap();
    let id = regex
      .captures(heading)
      .unwrap()
      .get(1)
      .unwrap()
      .as_str()
      .parse::<usize>()
      .unwrap();
    for beacon in lines {
      let coords = beacon.split(',').map(|l| l.parse::<isize>().unwrap());
      beacons.push(Vector3::from_iterator(coords));
    }
    scanners.push(Scanner { id, beacons })
  }
  scanners
}
