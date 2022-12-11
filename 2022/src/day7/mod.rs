use std::{collections::HashMap, fs};

const TEST: &str = r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

struct FileSystem {
  nodes: HashMap<String, Node>,
  cwd: String,
}

#[derive(Debug)]
enum Node {
  Dir(Dir),
  File(File),
}

#[derive(Debug)]
struct Dir {
  children: Vec<String>,
}

#[derive(Debug)]
struct File {
  size: usize,
}

impl FileSystem {
  fn new() -> Self {
    Self {
      nodes: HashMap::new(),
      cwd: "".to_string(),
    }
  }

  fn cd(&mut self, path: &str) {
    let mut path = path.to_string();
    if path.starts_with('/') {
      self.cwd = "".to_string();
      path = path[1..].to_string();
    }
    for dir in path.split('/') {
      if dir == ".." {
        self.cwd = self.cwd[..self.cwd.rfind('/').unwrap()].to_string();
      } else {
        self.cwd = format!("{}/{}", self.cwd, dir);
      }
    }
  }

  fn add(&mut self, path: &str, node: Node) {
    let path = if path.starts_with("//") {
      &path[1..]
    } else {
      path
    };
    self.nodes.insert(path.to_string(), node);
    let mut parent = path[..path.rfind('/').unwrap()].to_string();
    if parent.is_empty() {
      parent = "/".to_string();
    }
    let parent_node = self.nodes.get_mut(&parent).unwrap();

    if let Node::Dir(dir) = parent_node {
      if path == "/" {
        return;
      }
      dir.children.push(path.to_string());
    }
  }

  fn node_size(&self, path: &str) -> usize {
    let node = self.nodes.get(path).unwrap();
    match node {
      Node::Dir(dir) => dir.children.iter().map(|child| self.node_size(child)).sum(),
      Node::File(file) => file.size,
    }
  }

  fn sum_of_nodes_under_limit(&self, limit: usize) -> usize {
    self
      .nodes
      .iter()
      .filter_map(|(_, node)| match node {
        Node::Dir(dir) => {
          let size = dir
            .children
            .iter()
            .map(|child| self.node_size(child))
            .sum::<usize>();
          if size < limit {
            Some(size)
          } else {
            None
          }
        }
        Node::File(_) => None,
      })
      .sum()
  }

  fn find_smallest_directory_over_limit(&self, limit: usize) -> usize {
    let mut smallest_size = usize::MAX;
    for (_, node) in self.nodes.iter() {
      if let Node::Dir(dir) = node {
        let size = dir
          .children
          .iter()
          .map(|child| self.node_size(child))
          .sum::<usize>();
        if size > limit && size < smallest_size {
          smallest_size = size;
        }
      }
    }
    smallest_size
  }
}

impl Node {
  fn new_dir() -> Self {
    Self::Dir(Dir {
      children: Vec::new(),
    })
  }

  fn new_file(size: usize) -> Self {
    Self::File(File { size })
  }
}

pub fn main() {
  let test = parse(TEST);
  let test1 = test.sum_of_nodes_under_limit(100000);
  assert_eq!(test1, 95437);
  println!("Day 7: Test 1: {}", test1);

  let input = parse(&fs::read_to_string("data/day7.txt").unwrap());
  let part1 = input.sum_of_nodes_under_limit(100000);
  assert_eq!(part1, 1908462);
  println!("Day 7: Part 1: {}", part1);

  let limit = test.node_size("/") - 40000000;
  let test2 = test.find_smallest_directory_over_limit(limit);
  assert_eq!(test2, 24933642);
  println!("Day 7: Test 2: {}", test2);

  let limit = input.node_size("/") - 40000000;
  let part2 = input.find_smallest_directory_over_limit(limit);
  assert_eq!(part2, 3979145);
  println!("Day 7: Part 2: {}", part2);
}

fn parse(input: &str) -> FileSystem {
  let mut fs = FileSystem::new();
  fs.add("/", Node::new_dir());
  for line in input.lines() {
    let mut parts = line.split_whitespace();
    let first = parts.next().unwrap();
    let second = parts.next().unwrap();
    match first {
      "$" => {
        if second == "cd" {
          fs.cd(parts.next().unwrap())
        }
      }
      "dir" => {
        fs.add(&format!("{}/{}", fs.cwd, second), Node::new_dir());
      }
      _ => {
        fs.add(
          &format!("{}/{}", fs.cwd, second),
          Node::new_file(first.parse().unwrap()),
        );
      }
    }
  }
  fs
}
