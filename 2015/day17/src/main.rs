fn main() {
    let test = vec![20, 15, 10, 5, 5];
    let containers = vec![11, 30, 47, 31, 32, 36, 3, 1, 5, 3, 32, 36, 15, 11, 46, 26, 28, 1, 19, 3];

    let test_out = solve(test, 25);
    assert_eq!(test_out, 4);
    println!("Test: {} unique combinations", test_out);

    let part1 = solve(containers, 150);
    assert_eq!(part1, 4372);
    println!("Test: {} unique combinations", part1);
}

fn solve(containers: Vec<i32>, volume: i32) -> i32 {
    let mut solutions = 0;
    for x in 1..2_i32.pow(containers.len() as u32) - 1 {
        let mut fill = 0;
        for y in 0..containers.len() {
            if x & 2_i32.pow(y as u32) != 0 {
                fill += containers.get(y).unwrap();
            }
        }
        if fill == volume {
            solutions += 1;
        }
    }
    solutions
}
