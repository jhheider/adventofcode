use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs;

pub fn main() {
    let test = fs::read_to_string("data/day14test.txt").unwrap();
    let input = fs::read_to_string("data/day14.txt").unwrap();

    let test1 = compute(&test, 1000, false);
    assert_eq!(test1, 1120);
    println!("Day 14: Test 1: winner is at {}km", test1);

    let part1 = compute(&input, 2503, false);
    assert_eq!(part1, 2640);
    println!("Day 14: Part 1: winner is at {}km", part1);

    let test2 = compute(&test, 1000, true);
    assert_eq!(test2, 689);
    println!("Day 14: Test 2: winner is at {} points", test2);

    let part2 = compute(&input, 2503, true);
    assert_eq!(part2, 1102);
    println!("Day 14: Part 2: winner is at {} points", part2);
}

fn compute(input: &str, seconds: i32, part2: bool) -> i32 {
    let parser = Regex::new(
        "(.+) can fly ([0-9]+) km/s for ([0-9]+) seconds, but then must rest for ([0-9]+) seconds.",
    )
    .unwrap();
    let mut speeds = HashMap::new();

    for i in input.lines() {
        let caps = parser.captures(i).unwrap();

        let name = caps.get(1).unwrap().as_str();
        let speed = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let time_on = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let time_off = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();
        let period = time_on + time_off;

        speeds.insert(name, (speed, time_on, period));
    }

    if !part2 {
        return calc(&speeds, seconds).0;
    }

    let mut scores = HashMap::new();
    for s in 1..=seconds {
        let (winning, distance) = calc(&speeds, s);
        for (name, d) in distance.iter() {
            if *d == winning {
                scores.insert(*name, scores.get(name).unwrap_or(&0) + 1);
            }
        }
    }

    scores.iter().fold(0, |a, b| max(a, *b.1))
}

fn calc<'a>(speeds: &'a HashMap<&str, (i32, i32, i32)>, time: i32) -> (i32, HashMap<&'a str, i32>) {
    let mut winning = 0;
    let mut distance = HashMap::new();

    for (name, speed) in speeds.iter() {
        let mut dx = (time / speed.2) * speed.0 * speed.1;

        dx += min(time % speed.2, speed.1) * speed.0;
        distance.insert(*name, dx);
        if dx > winning {
            winning = dx;
        };
    }

    (winning, distance)
}
