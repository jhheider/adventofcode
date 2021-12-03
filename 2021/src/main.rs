use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

fn main() {
    let days = [day1::main,
        day2::main,
        day3::main,
        day4::main,
        day5::main,
        day6::main,
        day7::main,
        day8::main,
        day9::main,
        day10::main,
        day11::main,
        day12::main,
        day13::main,
        day14::main,
        day15::main,
        day16::main,
        day17::main,
        day18::main,
        day19::main,
        day20::main,
        day21::main,
        day22::main,
        day23::main,
        day24::main,
        day25::main];

    match env::args().collect::<Vec<String>>().get(1) {
        Some(n) => days[n.parse::<usize>().unwrap() - 1](),
        None => *days.iter().map(|func| func()).collect::<Vec<_>>().last().unwrap(),
    };
}
