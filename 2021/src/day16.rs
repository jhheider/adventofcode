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
  let input = "020D708041258C0B4C683E61F674A1401595CC3DE669AC4FB7BEFEE840182CDF033401296F44367F938371802D2CC9801A980021304609C431007239C2C860400F7C36B005E446A44662A2805925FF96CBCE0033C5736D13D9CFCDC001C89BF57505799C0D1802D2639801A900021105A3A43C1007A1EC368A72D86130057401782F25B9054B94B003013EDF34133218A00D4A6F1985624B331FE359C354F7EB64A8524027D4DEB785CA00D540010D8E9132270803F1CA1D416200FDAC01697DCEB43D9DC5F6B7239CCA7557200986C013912598FF0BE4DFCC012C0091E7EFFA6E44123CE74624FBA01001328C01C8FF06E0A9803D1FA3343E3007A1641684C600B47DE009024ED7DD9564ED7DD940C017A00AF26654F76B5C62C65295B1B4ED8C1804DD979E2B13A97029CFCB3F1F96F28CE43318560F8400E2CAA5D80270FA1C90099D3D41BE00DD00010B893132108002131662342D91AFCA6330001073EA2E0054BC098804B5C00CC667B79727FF646267FA9E3971C96E71E8C00D911A9C738EC401A6CBEA33BC09B8015697BB7CD746E4A9FD4BB5613004BC01598EEE96EF755149B9A049D80480230C0041E514A51467D226E692801F049F73287F7AC29CB453E4B1FDE1F624100203368B3670200C46E93D13CAD11A6673B63A42600C00021119E304271006A30C3B844200E45F8A306C8037C9CA6FF850B004A459672B5C4E66A80090CC4F31E1D80193E60068801EC056498012804C58011BEC0414A00EF46005880162006800A3460073007B620070801E801073002B2C0055CEE9BC801DC9F5B913587D2C90600E4D93CE1A4DB51007E7399B066802339EEC65F519CF7632FAB900A45398C4A45B401AB8803506A2E4300004262AC13866401434D984CA4490ACA81CC0FB008B93764F9A8AE4F7ABED6B293330D46B7969998021C9EEF67C97BAC122822017C1C9FA0745B930D9C480";

  let test1 = tests1
    .iter()
    .map(|test| parse(test))
    .collect::<Vec<usize>>();
  assert_eq!(test1, vec![6, 9, 14, 16, 12, 23, 31]);
  for (index, test) in test1.iter().enumerate() {
    println!("Day 16: Test {}: total of versions is {}", index + 1, test);
  }

  let part1 = parse(input);
  assert_eq!(part1, 965);
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

  let part2 = compute(input);
  assert_eq!(part2, 116672213160);
  println!("Day 16: Part 1: total of versions is {}", part2);
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
