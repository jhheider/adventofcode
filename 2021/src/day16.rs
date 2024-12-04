use crate::data::Data;

#[derive(Debug)]
struct Packet {
  version: usize,
  id: usize,
  value: usize,
  children: Vec<Packet>,
}

impl Packet {
  fn versions(&self) -> usize {
    self.version + self.children.iter().map(|c| c.versions()).sum::<usize>()
  }

  fn compute(&self) -> usize {
    match self.id {
      0 => self.children.iter().map(|c| c.compute()).sum(),
      1 => self.children.iter().map(|c| c.compute()).product(),
      2 => self.children.iter().map(|c| c.compute()).min().unwrap(),
      3 => self.children.iter().map(|c| c.compute()).max().unwrap(),
      4 => self.value,
      5 => (self.children[0].compute() > self.children[1].compute()) as usize,
      6 => (self.children[0].compute() < self.children[1].compute()) as usize,
      7 => (self.children[0].compute() == self.children[1].compute()) as usize,
      _ => panic!("Invalid byte: {}", self.id),
    }
  }
}

pub fn main() {
  let tests1 = [
    "D2FE28",
    "38006F45291200",
    "EE00D40C823060",
    "8A004A801A8002F478",
    "620080001611562C8802118E34",
    "C0015000016115A2E0802F182340",
    "A0016C880162017C3686B18A3D4780",
  ];
  let data = Data::get(16);

  let test1 = tests1
    .iter()
    .map(|test| parse(test))
    .collect::<Vec<usize>>();
  assert_eq!(test1, vec![6, 9, 14, 16, 12, 23, 31]);
  for (index, test) in test1.iter().enumerate() {
    println!("Day 16: Test {}: total of versions is {}", index + 1, test);
  }

  let part1 = parse(&data.input);
  println!("Day 16: Part 1: total of versions is {}", part1);

  let tests2 = [
    "C200B40A82",
    "04005AC33890",
    "880086C3E88112",
    "CE00C43D881120",
    "D8005AC2A8F0",
    "F600BC2D8F",
    "9C005AC2F8F0",
    "9C0141080250320F1802104A08",
  ];

  let test2 = tests2
    .iter()
    .map(|test| compute(test))
    .collect::<Vec<usize>>();
  assert_eq!(test2, vec![3, 54, 7, 9, 1, 0, 0, 1]);
  for (index, test) in test2.iter().enumerate() {
    println!(
      "Day 16: Test {}: value of calculation is {}",
      index + 8,
      test
    );
  }

  let part2 = compute(&data.input);
  println!("Day 16: Part 2: total of versions is {}", part2);
}

fn parse(input: &str) -> usize {
  let buffer = input.chars().fold("".to_string(), |s, c| {
    s + &format!("{:04b}", c.to_digit(16).unwrap())
  });
  let root = debuffer_packets(buffer);
  root.0.versions()
}

fn debuffer_packets(mut buffer: String) -> (Packet, String) {
  let version = str_to_value(&buffer[..3]);
  let id = str_to_value(&buffer[3..6]);
  buffer = buffer[6..].to_string();

  if id == 4 {
    let mut value = String::new();
    loop {
      let last = &buffer[..1] == "0";
      value += &buffer[1..5];
      buffer = buffer[5..].to_string();
      if last {
        break;
      }
    }
    (
      Packet {
        version,
        id,
        value: str_to_value(&value),
        children: Vec::new(),
      },
      buffer,
    )
  } else {
    match &buffer[..1] {
      "0" => {
        let length = str_to_value(&buffer[1..16]);
        let mut sub_buffer = buffer[16..16 + length].to_string();
        buffer = buffer[16 + length..].to_string();
        let mut children = Vec::new();
        while sub_buffer.contains('1') {
          let rv = debuffer_packets(sub_buffer);
          children.push(rv.0);
          sub_buffer = rv.1;
        }
        (
          Packet {
            version,
            id,
            value: 0,
            children,
          },
          buffer,
        )
      }
      "1" => {
        let value = str_to_value(&buffer[1..12]);
        let mut children = Vec::new();
        buffer = buffer[12..].to_string();
        for _ in 0..value {
          let rv = debuffer_packets(buffer);
          children.push(rv.0);
          buffer = rv.1;
        }
        (
          Packet {
            version,
            id,
            value: 0,
            children,
          },
          buffer,
        )
      }
      b => panic!("Impossible bit: {}", b),
    }
  }
}

fn str_to_value(s: &str) -> usize {
  usize::from_str_radix(s, 2).unwrap()
}

fn compute(input: &str) -> usize {
  let root = debuffer_packets(input.chars().fold("".to_string(), |s, c| {
    s + &format!("{:04b}", c.to_digit(16).unwrap())
  }))
  .0;

  root.compute()
}
