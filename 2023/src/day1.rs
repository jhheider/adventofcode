use crate::data::Data;

const TEST: &str = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

pub fn main() {
  let data = Data::get(1);
  let test = compute_sum(TEST, false);
  println!("Day 1, Test: {}", test);

  println!("Day 1, Part 1: {}", compute_sum(&data.input, false));

  let test2 = compute_sum(&data.test, true);
  println!("Day 1, Test2: {}", test2);

  println!("Day 1, Part 2: {}", compute_sum(&data.input, true));
}

fn compute_sum(input: &str, part2: bool) -> u32 {
  let mut sum = 0;
  for line in input.lines() {
    let mut numbers = Vec::new();
    let chars = line.chars().collect::<Vec<char>>();
    let len = line.len();
    for i in 0..len {
      match chars[i] {
        c if c.is_numeric() => numbers.push(c.to_digit(10).unwrap()),
        'o' if part2 => {
          if i + 2 < len && chars[i + 1] == 'n' && chars[i + 2] == 'e' {
            numbers.push(1);
          }
        }
        't' if part2 => {
          if i + 2 < len && chars[i + 1] == 'w' && chars[i + 2] == 'o' {
            numbers.push(2);
          } else if i + 4 < len
            && chars[i + 1] == 'h'
            && chars[i + 2] == 'r'
            && chars[i + 3] == 'e'
            && chars[i + 4] == 'e'
          {
            numbers.push(3);
          }
        }
        'f' if part2 => {
          if i + 3 < len && chars[i + 1] == 'o' && chars[i + 2] == 'u' && chars[i + 3] == 'r' {
            numbers.push(4);
          } else if i + 3 < len && chars[i + 1] == 'i' && chars[i + 2] == 'v' && chars[i + 3] == 'e'
          {
            numbers.push(5);
          }
        }
        's' if part2 => {
          if i + 2 < len && chars[i + 1] == 'i' && chars[i + 2] == 'x' {
            numbers.push(6);
          } else if i + 4 < len
            && chars[i + 1] == 'e'
            && chars[i + 2] == 'v'
            && chars[i + 3] == 'e'
            && chars[i + 4] == 'n'
          {
            numbers.push(7);
          }
        }
        'e' if part2 => {
          if i + 4 < len
            && chars[i + 1] == 'i'
            && chars[i + 2] == 'g'
            && chars[i + 3] == 'h'
            && chars[i + 4] == 't'
          {
            numbers.push(8);
          }
        }
        'n' if part2 => {
          if i + 3 < len && chars[i + 1] == 'i' && chars[i + 2] == 'n' && chars[i + 3] == 'e' {
            numbers.push(9);
          }
        }
        _ => (),
      }
    }
    if !numbers.is_empty() {
      sum += 10 * numbers[0] + numbers[numbers.len() - 1];
      // eprintln!("{}: {:?}, {}", line, numbers, sum);
    }
  }
  sum
}
