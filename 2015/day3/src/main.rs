use std::fs;
use std::cmp::Eq;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn next(self, dir: char) -> Position {
        match dir {
            '^' => Position { x: self.x, y: self.y + 1 },
            'v' => Position { x: self.x, y: self.y - 1 },
            '<' => Position { x: self.x - 1, y: self.y },
            '>' => Position { x: self.x + 1, y: self.y },
            _ => panic!("Bad input"),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut position = Position { x: 0, y: 0 };
    let mut houses = HashSet::new();

    houses.insert(position);

    for next_move in input.chars() {
        position = position.next(next_move);

        houses.insert(position);
    }

    println!("Part 1: Santa visited {} houses at least once.", houses.len());

    let mut positions = [Position { x: 0, y: 0 }, Position { x: 0, y: 0 }];

    houses.clear();
    houses.insert(positions[0]);

    for (i, next_move) in input.chars().enumerate() {
        positions[i % 2] = positions[i % 2].next(next_move);
        houses.insert(positions[i % 2]);
    }

    println!("Part 2: Santa and Robo-Santa visited {} houses at least once.", houses.len());
}
