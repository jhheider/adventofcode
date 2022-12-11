use itertools::Itertools;

#[derive(Debug, Clone)]
struct Packages {
    packages: Vec<usize>,
}

impl Packages {
    fn new(packages: Vec<usize>) -> Self {
        Self { packages }
    }

    fn qe(&self) -> usize {
        self.packages.iter().product()
    }

    fn smallest_group(&self, group_size: usize) -> usize {
        let total_weight = self.packages.iter().sum::<usize>();
        let target_weight = total_weight / group_size;
        for size in 1.. {
            let found = self
                .packages
                .clone()
                .into_iter()
                .combinations(size)
                .filter_map(|c| {
                    if c.iter().sum::<usize>() == target_weight {
                        Some(Packages { packages: c })
                    } else {
                        None
                    }
                })
                .collect_vec();
            if !found.is_empty() {
                return found.iter().map(|c| c.qe()).min().unwrap();
            }
        }
        usize::MAX
    }
}

pub fn main() {
    let test = Packages::new(vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11]);
    let test1 = test.smallest_group(3);
    assert_eq!(test1, 99);
    println!("Day 24: Test 1: {:?}", test1);

    let input = Packages::new(vec![
        1, 3, 5, 11, 13, 17, 19, 23, 29, 31, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
        101, 103, 107, 109, 113,
    ]);
    let part1 = input.smallest_group(3);
    assert_eq!(part1, 11266889531);
    println!("Day 24: Part 1: {:?}", part1);

    let test2 = test.smallest_group(4);
    assert_eq!(test2, 44);
    println!("Day 24: Test 2: {:?}", test2);

    let part2 = input.smallest_group(4);
    assert_eq!(part2, 77387711);
    println!("Day 24: Part 2: {:?}", part2);
}
