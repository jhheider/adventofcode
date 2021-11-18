use std::cmp::min;
fn main() {
    let test = vec![20, 15, 10, 5, 5];
    let containers = vec![11, 30, 47, 31, 32, 36, 3, 1, 5, 3, 32, 36, 15, 11, 46, 26, 28, 1, 19, 3];

    let test_out = solve(test, 25);
    assert_eq!(test_out.0, 4);
    println!("Test: {} unique combinations", test_out.0);

    let part1 = solve(containers, 150);
    assert_eq!(part1.0, 4372);
    println!("Part 1: {} unique combinations", part1.0);

    assert_eq!(test_out.1, 3);
    println!("Test: {} permuatations", test_out.1);

    assert_eq!(part1.1, 4);
    println!("Part 2: {} unique combinations", part1.1);
}

fn solve(containers: Vec<i32>, volume: i32) -> (usize, i32) {
    let mut solutions = Vec::new();
    let mut least = containers.len();
    for x in 1..2_i32.pow(containers.len() as u32) - 1 {
        let mut fill = 0;
        let mut used = 0;
        for y in 0..containers.len() {
            if x & 2_i32.pow(y as u32) != 0 {
                fill += containers.get(y).unwrap();
                used += 1;
            }
        }
        if fill == volume {
            solutions.push(used);
            least = min(least, used);
        }
    }

    let permutations = solutions.iter().fold(0, |total, used| {
        if *used == least { return total + 1 }
        total
    });
    (solutions.len(), permutations)
}
