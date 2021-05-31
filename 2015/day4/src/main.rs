use md5;

fn main() {
    let input = String::from("yzbqklnj");

    let mut tail = 1;

    loop {
        let sum = md5::compute(String::from(&input) + &tail.to_string());

        if sum[0] as i32 + sum[1] as i32 + (sum[2] >> 4) as i32 == 0 {
            break;
        }

        tail += 1;
    };

    println!("Part 1: the lowest number is {}", tail);

    tail = 1;

    loop {
        let sum = md5::compute(String::from(&input) + &tail.to_string());

        if sum[0] as i32 + sum[1] as i32 + sum[2] as i32 == 0 {
            break;
        }

        tail += 1;
    };

    println!("Part 2: the lowest number is {}", tail);
}