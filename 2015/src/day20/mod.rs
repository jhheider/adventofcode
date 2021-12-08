pub fn main() {
  let test = 150;
  let input = 36000000;

  let test1 = lowest_candidate(test,false);
  assert_eq!(test1, 8);
  println!("Day 20: Test 1: the first house to have {} presents is #{}", test, test1);

  let part1 = lowest_candidate(input, false);
  assert_eq!(part1, 831600);
  println!("Day 20: Part 1: the first house to have {} presents is #{}", input, part1);

  let test2 = lowest_candidate(test,true);
  assert_eq!(test2, 8);
  println!("Day 20: Test 2: the first house to have {} presents is #{}", test, test2);

  let part2 = lowest_candidate(input, true);
  assert_eq!(part2, 884520);
  println!("Day 20: Part 2: the first house to have {} presents is #{}", input, part2);
}

fn lowest_candidate(threshold: i32, lazy_elves: bool) -> i32 {
  for house in 1.. {
    let mut presents = 0;
    for elf in 1..=(house as f64).sqrt() as i32 {
      if house % elf == 0 {
        if lazy_elves {
          if house / elf < 50 { presents += 11 * elf }
          if elf < 50 { presents += 11 * house / elf }
        } else {
          presents += 11 * elf + 10 * house / elf;
        }
      }
    }
    if presents >= threshold { return house }
  }
  -1
}