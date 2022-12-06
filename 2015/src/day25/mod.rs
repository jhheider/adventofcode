pub fn main() {
    let part1 = nth(index(3010, 3019));
    assert_eq!(part1, 8997277);
    println!("Day 25, Part 1: {}", part1);
}

fn index(a: usize, b: usize) -> usize {
    let corner = a + b - 1;
    let corner_val = (corner * (corner - 1)) / 2 + 1;
    corner_val + b - 1
}

fn nth(n: usize) -> usize {
    let mut code = 20151125;
    for _ in 1..n {
        code = (code * 252533) % 33554393;
    }
    code
}
