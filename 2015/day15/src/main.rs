use std::fs;
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
    let _input = fs::read_to_string("input.txt").unwrap().vectorize();

    let test = solve(&test);
    assert_eq!(test, 62842880);
    println!("Test: best cookie scores {}", test);
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

fn solve(_input: &[Ingredient]) -> i32 {
    todo!()
}