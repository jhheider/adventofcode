use std::fs;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut lights = [[false; 1000]; 1000];

    for command in input.lines() {
        let command = parse_command(command);
        for row in lights.iter_mut().take(command.2 + 1).skip(command.1) {
            for light in row.iter_mut().take(command.4 + 1).skip(command.3) {
                match command.0 {
                    "turn on" => *light = true,
                    "turn off" => *light = false,
                    "toggle" => *light = !*light,
                    _ => panic!("Bad input: {}", command.0)
                }
            }
        }
    }

    let lit = lights.iter().flatten().filter(|x| **x).count();

    assert_eq!(lit, 377891);
    println!("Part 1: {} lights lit.", lit);

    let mut lights = [[0; 1000]; 1000];

    for command in input.lines() {
        let command = parse_command(command);
        for row in lights.iter_mut().take(command.2 + 1).skip(command.1) {
            for light in row.iter_mut().take(command.4 + 1).skip(command.3) {
                match command.0 {
                    "turn on" => *light += 1,
                    "turn off" => if *light > 0 { *light -= 1 },
                    "toggle" => *light += 2,
                    _ => panic!("Bad input: {}", command.0)
                }
            }
        }
    }

    let brightness: i32 = lights.iter().flatten().sum();

    assert_eq!(brightness, 14110788);
    println!("Part 2: {:?} total brightness.", brightness);
}

fn parse_command(command: &str) -> (&str, usize, usize, usize, usize) {
    let row = Regex::new(r"^(turn off|turn on|toggle) (.*),(.*) through (.*),(.*)$").unwrap();

    let captures = row.captures(command).unwrap();
    let command = captures.get(1).unwrap().as_str();
    let startx = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
    let starty = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
    let endx = captures.get(4).unwrap().as_str().parse::<usize>().unwrap();
    let endy = captures.get(5).unwrap().as_str().parse::<usize>().unwrap();

    (command, startx, endx, starty, endy)
}