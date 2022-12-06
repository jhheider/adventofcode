use std::{collections::HashMap, fs};

pub fn main() {
    let test = fs::read_to_string("data/day18test.txt").unwrap();
    let input = fs::read_to_string("data/day18.txt").unwrap();

    let test1 = solve(test.clone(), 4, false);
    assert_eq!(test1, 4);
    println!(
        "Day 18: Test 1: input has {} lights after {} iterations",
        test1, 4
    );

    let part1 = solve(input.clone(), 100, false);
    assert_eq!(part1, 768);
    println!(
        "Day 18: Part 1: input has {} lights after {} iterations",
        part1, 100
    );

    let test2 = solve(test, 5, true);
    assert_eq!(test2, 17);
    println!(
        "Day 18: Test 2: input has {} lights after {} iterations",
        test2, 5
    );

    let part2 = solve(input, 100, true);
    assert_eq!(part2, 781);
    println!(
        "Day 18: Part 2: input has {} lights after {} iterations",
        part2, 100
    );
}

fn solve(input: String, iterations: u32, part2: bool) -> u32 {
    let mut old_state = HashMap::new();
    let size = input.lines().count() as i32;

    for (x, line) in input.lines().enumerate() {
        for (y, cell) in line.chars().enumerate() {
            if part2 && (x == 0 || x == (size as usize) - 1) && (y == 0 || y == (size as usize) - 1)
            {
                old_state.insert((x as i32, y as i32), 1);
                continue;
            }
            match cell {
                '#' => old_state.insert((x as i32, y as i32), 1),
                '.' => old_state.insert((x as i32, y as i32), 0),
                _ => panic!("Invalid state detected"),
            };
        }
    }

    for _ in 0..iterations {
        let mut new_state = HashMap::new();
        for x in 0..size {
            for y in 0..size {
                if part2 && (x == 0 || x == size - 1) && (y == 0 || y == size - 1) {
                    new_state.insert((x, y), 1);
                    continue;
                }
                let current = old_state.get(&(x, y)).unwrap();
                let neighbors = old_state.get(&(x - 1, y - 1)).unwrap_or(&0)
                    + old_state.get(&(x, y - 1)).unwrap_or(&0)
                    + old_state.get(&(x + 1, y - 1)).unwrap_or(&0)
                    + old_state.get(&(x - 1, y)).unwrap_or(&0)
                    + old_state.get(&(x + 1, y)).unwrap_or(&0)
                    + old_state.get(&(x - 1, y + 1)).unwrap_or(&0)
                    + old_state.get(&(x, y + 1)).unwrap_or(&0)
                    + old_state.get(&(x + 1, y + 1)).unwrap_or(&0);
                if neighbors == 3 || (neighbors == 2 && *current == 1) {
                    new_state.insert((x, y), 1);
                } else {
                    new_state.insert((x, y), 0);
                }
            }
        }
        old_state = new_state;
    }
    old_state.values().sum()
}
