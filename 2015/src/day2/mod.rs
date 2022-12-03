use std::cmp::min;
use std::fs;

pub fn main() {
    let input = fs::read_to_string("data/day2.txt").unwrap();

    let mut paper = 0;

    for present in input.lines() {
        let dims: Vec<i32> = present
            .split('x')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let a = dims.first().unwrap() * dims.get(1).unwrap();
        let b = dims.get(1).unwrap() * dims.get(2).unwrap();
        let c = dims.get(2).unwrap() * dims.first().unwrap();
        paper += 2 * a + 2 * b + 2 * c + min(min(a, b), c);
    }

    assert_eq!(paper, 1588178);
    println!("Day 2: Part 1: total square feet of paper = {}", paper);

    let mut ribbon = 0;

    for present in input.lines() {
        let dims: Vec<i32> = present
            .split('x')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let a = dims.first().unwrap();
        let b = dims.get(1).unwrap();
        let c = dims.get(2).unwrap();

        let perimeter = 2 * min(min(a + b, b + c), c + a);
        let volume = a * b * c;

        ribbon += perimeter + volume;
    }

    assert_eq!(ribbon, 3783758);
    println!("Day 2: Part 2: total linear feet of ribbon = {}", ribbon);
}
