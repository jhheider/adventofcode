fn main() {
    let input = String::from("3113322113");

    let test = [
        ("1", "11"),
        ("11", "21"),
        ("21", "1211"),
        ("1211", "111221"),
        ("111221", "312211"),
    ];

    for t in test.iter() {
        assert_eq!(iterate(String::from(t.0)), t.1);
    }

    let mut part1 = input;

    for _ in 0..40 {
        part1 = iterate(part1);
    }

    assert_eq!(part1.len(), 329356);
    println!("Part 1: {} chars", part1.len());

    let mut part2 = part1;

    for _ in 40..50 {
        part2 = iterate(part2);
    }

    assert_eq!(part2.len(), 4666278);
    println!("Part 2: {} chars", part2.len());
}

fn iterate(s: String) -> String {
    let mut count = 0;
    let mut last = String::from("a");
    let mut output = String::new();
    for c in s.chars() {
        let c = c.to_string();
        if last != c {
            if last != "a" {
                output += &(count.to_string() + &last.to_string());
            }
            last = c.clone();
            count = 0;
        }
        count += 1;
    }
    output + &count.to_string() + &last.to_string()
}