use crate::data::Data;

pub fn main() {
    let data = Data::get(1);
    let input = &data.input;

    let mut floor = 0;

    for c in input.chars() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => panic!("Bad input"),
        };
    }

    println!("Day 1: Part 1: final floor = {}", floor);

    floor = 0;

    for (i, c) in input.chars().enumerate() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => panic!("Bad input"),
        };
        if floor < 0 {
            println!("Day 1: Part 2: Entered the basement on character {}", i + 1);
            return;
        }
    }
}
