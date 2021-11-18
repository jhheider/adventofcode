use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut floor = 0;

    for c in input.chars() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => panic!("Bad input"),
        };
    }

    assert_eq!(floor, 280);
    println!("Part 1: final floor = {}", floor);

    floor = 0;

    for (i, c) in input.chars().enumerate() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => panic!("Bad input"),
        };
        if floor < 0 {
            assert_eq!(i + 1, 1797);
            println!("Part 2: Entered the basement on character {}", i + 1);
            return
        }
    }
}
