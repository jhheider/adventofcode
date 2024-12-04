use std::{fmt::Debug, fs};

#[derive(Clone)]
struct SFNumber {
  value: Vec<SFValue>,
}

#[derive(Clone, Debug, PartialEq)]
enum SFValue {
  Open,
  Close,
  Value(isize),
}

impl SFValue {
  fn value(&self) -> isize {
    match self {
      SFValue::Open | SFValue::Close => 0,
      SFValue::Value(n) => *n,
    }
  }
}

impl Debug for SFNumber {
  fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    let mut output = String::new();
    let mut last = SFValue::Open;
    for v in self.value.clone() {
      output = match v {
        SFValue::Open => {
          if last == SFValue::Open {
            format!("{output}[")
          } else {
            format!("{output},[")
          }
        }
        SFValue::Close => format!("{output}]"),
        SFValue::Value(n) => {
          if last == SFValue::Open {
            format!("{output}{n}")
          } else {
            format!("{output},{n}")
          }
        }
      };
      last = v
    }
    write!(fmt, "{output}")
  }
}

impl SFNumber {
  fn from(s: &str) -> Self {
    let mut value = Vec::new();
    for c in s.chars() {
      match c {
        '[' => value.push(SFValue::Open),
        ']' => value.push(SFValue::Close),
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
          value.push(SFValue::Value(c.to_digit(10).unwrap() as isize));
        }
        _ => (),
      }
    }
    SFNumber { value }
  }

  fn add(&self, right: &SFNumber) -> Self {
    let mut value = vec![SFValue::Open];
    value.extend(self.clone().value);
    value.extend(right.clone().value);
    value.push(SFValue::Close);
    SFNumber { value }.reduce()
  }

  fn reduce(&self) -> Self {
    let mut depth = 0;
    let mut should_split = None;
    for i in 0..self.value.len() {
      match self.value[i] {
        SFValue::Open => {
          if depth < 4 {
            depth += 1;
          } else {
            // explode
            let mut value = self.value.clone();
            match (&self.value[i + 1], &self.value[i + 2]) {
              (SFValue::Value(left), SFValue::Value(right)) => {
                for j in (0..i).rev() {
                  if let SFValue::Value(n) = &value[j] {
                    value[j] = SFValue::Value(left + n);
                    break;
                  }
                }
                #[allow(clippy::needless_range_loop)]
                for k in i + 4..value.len() {
                  if let SFValue::Value(n) = &value[k] {
                    value[k] = SFValue::Value(right + n);
                    break;
                  }
                }
                value.splice(i..i + 4, vec![SFValue::Value(0)]);
                return SFNumber { value }.reduce();
              }
              _ => panic!("this is impossible"),
            }
          }
        }
        SFValue::Close => depth -= 1,
        SFValue::Value(n) => {
          if n > 9 && should_split.is_none() {
            should_split = Some(i);
          }
        }
      }
    }
    // split
    if let Some(i) = should_split {
      if let SFValue::Value(n) = self.value[i] {
        let mut value = self.value.clone();
        let left = n / 2;
        let right = (n + 1) / 2;
        value.splice(
          i..i + 1,
          vec![
            SFValue::Open,
            SFValue::Value(left),
            SFValue::Value(right),
            SFValue::Close,
          ],
        );
        return SFNumber { value }.reduce();
      }
    }
    self.clone()
  }

  fn magnitude(&self) -> isize {
    if self.value.len() == 1 {
      return self.value[0].value();
    }
    for i in 0..self.value.len() - 1 {
      if let (SFValue::Value(n), SFValue::Value(m)) = (&self.value[i], &self.value[i + 1]) {
        let mut value = self.value.clone();
        value.splice(i - 1..=i + 2, vec![SFValue::Value(3 * n + 2 * m)]);
        return SFNumber { value }.magnitude();
      }
    }
    panic!("can't get here");
  }
}

pub fn main() {
  let test = vec![
    "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
    "[[[5,[2,8]],4],[5,[[9,9],0]]]",
    "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
    "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
    "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
    "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
    "[[[[5,4],[7,7]],8],[[8,3],8]]",
    "[[9,3],[[9,9],[6,[4,9]]]]",
    "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
    "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
  ]
  .into_iter()
  .map(SFNumber::from)
  .collect::<Vec<SFNumber>>();

  let test_output = test
    .clone()
    .into_iter()
    .reduce(|total, next| total.add(&next))
    .unwrap();
  assert_eq!(test_output.magnitude(), 4140);
  println!(
    "Day 18: Part 1: test {test_output:?}, magnitude: {}",
    test_output.magnitude()
  );

  let input = fs::read_to_string("data/day18.txt")
    .unwrap()
    .lines()
    .map(SFNumber::from)
    .collect::<Vec<SFNumber>>();
  let part1 = input
    .clone()
    .into_iter()
    .reduce(|total, next| total.add(&next))
    .unwrap()
    .magnitude();
  assert_eq!(part1, 3806);
  println!("Day 18: Part 1: magnitude is {part1}");

  let test2 = max_magnitude(&test);
  assert_eq!(test2, 3993);
  println!("Day 18: Part 2: test magnitude is {test2}");

  let part2 = max_magnitude(&input);
  assert_eq!(part2, 4727);
  println!("Day 18: Part 2: magnitude is {part2}");
}

fn max_magnitude(input: &[SFNumber]) -> isize {
  let mut highest = isize::MIN;
  for i in 0..input.len() - 1 {
    for j in i + 1..input.len() {
      let left = input[i].add(&input[j]).magnitude();
      let right = input[j].add(&input[i]).magnitude();
      if left > highest {
        highest = left;
      }
      if right > highest {
        highest = right;
      }
    }
  }
  highest
}
