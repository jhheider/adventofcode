use std::fs;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut lights = [[false; 1000]; 1000];

    for command in input.lines() {
        let command = parse_command(command);
        for x in command.1..command.2 + 1 {
            for y in command.3..command.4 + 1 {
                match command.0 {
                    "turn on" => lights[x][y] = true,
                    "turn off" => lights[x][y] = false,
                    "toggle" => lights[x][y] = !lights[x][y],
                    _ => panic!("Bad input: {}", command.0)
                }
            }
        }
    }

    let lit = lights.iter().flatten().filter(|x| **x).count();

    println!("Part 1: {} lights lit.", lit);

    let mut lights = [[0; 1000]; 1000];

    for command in input.lines() {
        let command = parse_command(command);
        for x in command.1..command.2 + 1 {
            for y in command.3..command.4 + 1 {
                match command.0 {
                    "turn on" => lights[x][y] += 1,
                    "turn off" => if lights[x][y] > 0 { lights[x][y] -= 1 },
                    "toggle" => lights[x][y] += 2,
                    _ => panic!("Bad input: {}", command.0)
                }
            }
        }
    }

    let brightness = lights.iter().flatten().fold(0, |a, b| a + b);

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