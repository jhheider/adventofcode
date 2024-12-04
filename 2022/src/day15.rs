use std::collections::HashSet;

const TEST: &str = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

const INPUT: &str = r"Sensor at x=3772068, y=2853720: closest beacon is at x=4068389, y=2345925
Sensor at x=78607, y=2544104: closest beacon is at x=-152196, y=4183739
Sensor at x=3239531, y=3939220: closest beacon is at x=3568548, y=4206192
Sensor at x=339124, y=989831: closest beacon is at x=570292, y=1048239
Sensor at x=3957534, y=2132743: closest beacon is at x=3897332, y=2000000
Sensor at x=1882965, y=3426126: closest beacon is at x=2580484, y=3654136
Sensor at x=1159443, y=3861139: closest beacon is at x=2580484, y=3654136
Sensor at x=2433461, y=287013: closest beacon is at x=2088099, y=-190228
Sensor at x=3004122, y=3483833: closest beacon is at x=2580484, y=3654136
Sensor at x=3571821, y=799602: closest beacon is at x=3897332, y=2000000
Sensor at x=2376562, y=1539540: closest beacon is at x=2700909, y=2519581
Sensor at x=785113, y=1273008: closest beacon is at x=570292, y=1048239
Sensor at x=1990787, y=38164: closest beacon is at x=2088099, y=-190228
Sensor at x=3993778, y=3482849: closest beacon is at x=4247709, y=3561264
Sensor at x=3821391, y=3986080: closest beacon is at x=3568548, y=4206192
Sensor at x=2703294, y=3999015: closest beacon is at x=2580484, y=3654136
Sensor at x=1448314, y=2210094: closest beacon is at x=2700909, y=2519581
Sensor at x=3351224, y=2364892: closest beacon is at x=4068389, y=2345925
Sensor at x=196419, y=3491556: closest beacon is at x=-152196, y=4183739
Sensor at x=175004, y=138614: closest beacon is at x=570292, y=1048239
Sensor at x=1618460, y=806488: closest beacon is at x=570292, y=1048239
Sensor at x=3974730, y=1940193: closest beacon is at x=3897332, y=2000000
Sensor at x=2995314, y=2961775: closest beacon is at x=2700909, y=2519581
Sensor at x=105378, y=1513086: closest beacon is at x=570292, y=1048239
Sensor at x=3576958, y=3665667: closest beacon is at x=3568548, y=4206192
Sensor at x=2712265, y=2155055: closest beacon is at x=2700909, y=2519581";

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
  x: isize,
  y: isize,
}

impl Point {
  fn new(input: &str) -> Self {
    let mut iter = input.split(", ");
    let x = iter
      .next()
      .unwrap()
      .split('=')
      .nth(1)
      .unwrap()
      .parse()
      .unwrap();
    let y = iter
      .next()
      .unwrap()
      .split('=')
      .nth(1)
      .unwrap()
      .parse()
      .unwrap();
    Self { x, y }
  }

  fn distance(&self, other: &Self) -> isize {
    (self.x - other.x).abs() + (self.y - other.y).abs()
  }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Sensor {
  position: Point,
  closest_beacon: Point,
  beacon_distance: isize,
}

impl Sensor {
  fn new(input: &str) -> Self {
    let mut iter = input.split(": ");
    let position = Point::new(iter.next().unwrap().split(" at ").nth(1).unwrap());
    let closest_beacon = Point::new(iter.next().unwrap().split(" is at ").nth(1).unwrap());
    let beacon_distance = position.distance(&closest_beacon);
    Self {
      position,
      closest_beacon,
      beacon_distance,
    }
  }
}

struct Map {
  sensors: HashSet<Sensor>,
  beacons: HashSet<Point>,
  left: isize,
  right: isize,
}

impl Map {
  fn new(input: &str) -> Self {
    let mut sensors = HashSet::new();
    let mut beacons = HashSet::new();
    let mut left = isize::MAX;
    let mut right = isize::MIN;
    for line in input.lines() {
      let sensor = Sensor::new(line);
      sensors.insert(sensor);
      beacons.insert(sensor.closest_beacon);
      left = left.min(sensor.position.x - sensor.beacon_distance);
      right = right.max(sensor.position.x + sensor.beacon_distance);
    }
    Self {
      sensors,
      beacons,
      left,
      right,
    }
  }

  fn covered_point(&self, point: &Point) -> bool {
    for sensor in &self.sensors {
      if sensor.position.distance(point) <= sensor.beacon_distance {
        return true;
      }
    }
    false
  }

  fn covered_row(&self, row: isize) -> usize {
    let mut covered = HashSet::new();
    for x in self.left..=self.right {
      if self.beacons.contains(&Point { x, y: row }) {
        continue;
      }
      for sensor in &self.sensors {
        if sensor.position.distance(&Point { x, y: row }) <= sensor.beacon_distance {
          covered.insert(Point { x, y: row });
          break;
        }
      }
    }
    covered.len()
  }

  fn find_beacon(&self, x: isize, y: isize) -> Result<isize, &str> {
    for sensor in self.sensors.iter() {
      for dx in 0..=sensor.beacon_distance + 1 {
        let dy = sensor.beacon_distance - dx + 1;
        for p in [
          Point {
            x: sensor.position.x + dx,
            y: sensor.position.y + dy,
          },
          Point {
            x: sensor.position.x + dx,
            y: sensor.position.y - dy,
          },
          Point {
            x: sensor.position.x - dx,
            y: sensor.position.y + dy,
          },
          Point {
            x: sensor.position.x - dx,
            y: sensor.position.y - dy,
          },
        ]
        .iter()
        {
          if p.x < 0 || p.y < 0 || p.x > x || p.y > y {
            continue;
          }
          if !self.covered_point(p) {
            return Ok(p.x * 4_000_000 + p.y);
          }
        }
      }
    }
    Err("No beacon found")
  }
}

pub fn main() {
  let test = Map::new(TEST);
  assert_eq!(test.covered_row(10), 26);
  println!("Day 15: Test 1: {}", test.covered_row(10));

  let part1 = Map::new(INPUT);
  assert_eq!(part1.covered_row(2000000), 5299855);
  println!("Day 15: Part 1: {}", part1.covered_row(2000000));

  let test2 = test.find_beacon(20, 20).unwrap();
  assert_eq!(test2, 56_000_011);
  println!("Day 15: Test 2: {}", test2);

  let part2 = part1.find_beacon(4_000_000, 4_000_000).unwrap();
  assert_eq!(part2, 13_615_843_289_729);
  println!("Day 15: Part 2: {}", part2);
}
