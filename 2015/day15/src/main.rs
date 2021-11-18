use std::{fs, cmp::max};
use regex::Regex;

#[derive(Debug, Clone, Copy)]
pub struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn main() {
    let test = fs::read_to_string("test.txt").unwrap().vectorize();
    let input = fs::read_to_string("input.txt").unwrap().vectorize();

    let test = solve_test(&test);
    assert_eq!(test, 62842880);
    println!("Test: best cookie scores {}", test);

    let part1 = solve_input(&input, false);
    assert_eq!(part1, 18965440);
    println!("Part1: best cookie scores {}", part1);

    let part2 = solve_input(&input, true);
    assert_eq!(part2, 15862900);
    println!("Part1: best cookie scores {}", part2);
}

pub trait Vectorizable {
    fn vectorize(&self) -> Vec<Ingredient>;
}

impl Vectorizable for String {
    fn vectorize(&self) -> Vec<Ingredient> {
        let r = Regex::new(".+: capacity (.+), durability (.+), flavor (.+), texture (.+), calories (.+)").unwrap();

        self.lines().map(|l| {
            let caps = r.captures(l).unwrap();

            Ingredient {
                capacity: caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                durability: caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                flavor: caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                texture: caps.get(4).unwrap().as_str().parse::<i32>().unwrap(),
                calories: caps.get(5).unwrap().as_str().parse::<i32>().unwrap(),
            }
        }).collect()
    }
}

fn solve_test(input: &[Ingredient]) -> i32 {
    let mut best = -9999;
    for a in 0..100 {
        let cookie = max(a * input[0].capacity + (100 - a) * input[1].capacity, 0) *
            max(a * input[0].durability + (100 - a) * input[1].durability, 0) *
            max(a * input[0].flavor + (100 - a) * input[1].flavor, 0) *
            max(a * input[0].texture + (100 - a) * input[1].texture, 0);
        best = max(best, cookie);
    }
    best
}

fn solve_input(input: &[Ingredient], part2: bool) -> i32 {
    let mut best = -9999;
    for a in 0..100 {
        for b in 0..100-a {
            for c in 0..100-a-b {
                if part2 {
                    let calories = a * input[0].calories +
                        b * input[1].calories +
                        c * input[2].calories +
                        (100 - a - b - c) * input[3].calories;

                    if calories != 500 { continue; }
                }
                let capacity = max(
                    a * input[0].capacity +
                    b * input[1].capacity +
                    c * input[2].capacity +
                    (100 - a - b - c) * input[3].capacity, 0);

                let durability = max(
                    a * input[0].durability +
                    b * input[1].durability +
                    c * input[2].durability +
                    (100 - a - b - c) * input[3].durability, 0);

                let flavor = max(
                    a * input[0].flavor +
                    b * input[1].flavor +
                    c * input[2].flavor +
                    (100 - a - b - c) * input[3].flavor, 0);

                let texture = max(
                    a * input[0].texture +
                    b * input[1].texture +
                    c * input[2].texture +
                    (100 - a - b - c) * input[3].texture, 0);

                let cookie = capacity * durability * flavor * texture;
                best = max(best, cookie);
            }
        }
    }
    best
}
