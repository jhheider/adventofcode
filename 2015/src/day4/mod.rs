pub fn main() {
    let input = String::from("iwrupvqb");

    let mut tail = 1;

    loop {
        let sum = md5::compute(String::from(&input) + &tail.to_string());

        if sum[0] as i32 + sum[1] as i32 + (sum[2] >> 4) as i32 == 0 {
            break;
        }

        tail += 1;
    };

    assert_eq!(tail, 346386);
    println!("Day 4: Part 1: the lowest number is {}", tail);

    tail = 1;

    loop {
        let sum = md5::compute(String::from(&input) + &tail.to_string());

        if sum[0] as i32 + sum[1] as i32 + sum[2] as i32 == 0 {
            break;
        }

        tail += 1;
    };

    assert_eq!(tail, 9958218);
    println!("Day 4: Part 2: the lowest number is {}", tail);
}