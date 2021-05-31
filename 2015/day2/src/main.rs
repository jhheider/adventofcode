use std::fs;
use std::cmp::min;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut paper = 0;

    for present in input.lines() {
        let dims: Vec<i32> = present.split('x').map(|x| x.parse::<i32>().unwrap()).collect();
        let a = dims.get(0).unwrap() * dims.get(1).unwrap();
        let b = dims.get(1).unwrap() * dims.get(2).unwrap();
        let c = dims.get(2).unwrap() * dims.get(0).unwrap();
        paper += 2 * a + 2 * b + 2 * c + min(min(a, b,), c);
    }

    println!("Part 1: total square feet of paper = {}", paper);

    let mut ribbon = 0;

    for present in input.lines() {
        let dims: Vec<i32> = present.split('x').map(|x| x.parse::<i32>().unwrap()).collect();
        let a = dims.get(0).unwrap();
        let b = dims.get(1).unwrap();
        let c = dims.get(2).unwrap();

        let perimeter = 2 * min(min(a + b, b + c), c + a);
        let volume = a * b * c;

        ribbon += perimeter + volume;
    }

    println!("Part 2: total linear feet of ribbon = {}", ribbon);
}
