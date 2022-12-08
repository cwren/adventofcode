use lazy_static::lazy_static;
use regex::Regex;
use std::{fs::File, io::Read, str::Lines, borrow::BorrowMut};

#[derive(Clone)]
enum NodeFlavor {
    DIR,
    FILE,
}
use crate::NodeFlavor::{DIR, FILE};

#[derive(Clone)] 
struct Node {
    flavor: NodeFlavor,
    name: String,
    size: usize,
    nodes: Vec<Node>,
}

impl Node {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn size(&self) -> usize {
        match self.flavor {
            NodeFlavor::FILE => self.size,
            NodeFlavor::DIR => self.nodes.iter().map(|n| n.size()).sum(),
        }
    }

    fn len(&self) -> usize {
        match self.flavor {
            NodeFlavor::FILE => 1,
            NodeFlavor::DIR => self.nodes.len(),
        }
    }

    fn get(&self, probe: &str) -> Option<&Node> {
        for node in self.nodes.iter() {
            if node.name() == probe {
                return Some(&node);
            }
        }
        return None;
    }

    fn make_file(name: &str, size: usize) -> Node {
        Node {flavor: FILE, name: name.to_string(), size: size, nodes: Vec::new()}
    }

    fn make_dir(name: &str) -> Node {
        Node {flavor: DIR, name: name.to_string(), size: 0, nodes: Vec::new()}
    }
    
}

#[derive(Debug, PartialEq)]
enum Log {
    CD(String),
    LS,
    DIR(String),
    FILE(String, usize),
}

lazy_static! {
    static ref CD_RE: regex::Regex = Regex::new(r"^\$ cd ([/a-zA-Z0-9]+)$").unwrap();
    static ref LS_RE: regex::Regex = Regex::new(r"^\$ ls$").unwrap();
    static ref DIR_RE: regex::Regex = Regex::new(r"^dir ([/a-zA-Z0-9]+)$").unwrap();
    static ref FILE_RE: regex::Regex = Regex::new(r"^([0-9]+) ([/a-zA-Z0-9.]+)$").unwrap();
}

fn parse_log_line(log: &str) -> Log {
    if LS_RE.is_match(log) {
        return Log::LS;
    } else if CD_RE.is_match(log) {
        let cap = CD_RE.captures(log).expect("directory capture failure");
        let name = cap.get(1).expect("missing change directory capture").as_str().to_string();
        return Log::CD(name);

    } else if DIR_RE.is_match(log) {
        let cap = DIR_RE.captures(log).expect("directory capture failure");
        let name = cap.get(1).expect("missing directory name capture").as_str().to_string();
        return Log::DIR(name);
    } else if FILE_RE.is_match(log) {
        let cap = FILE_RE.captures(log).expect("file capture failure");
        let name = cap.get(2).expect("missing file name capture").as_str().to_string();
        let size = cap.get(1).unwrap().as_str().parse().expect("failed to parse file size");
        return Log::FILE(name, size);
    }
    panic!("unrecognized log line {log}");
}

fn parse_logs(input: &str) -> Node {
    let root = Node::make_dir("/");
    let mut path = Vec::new(); 
    for line in input.lines() {
        match parse_log_line(line) {
            Log::CD(name) => {
                if name == ".." {
                    path.pop();
                } else { 
                    if path.is_empty() {
                        path.push(Node::make_dir(&name)); // root node
                    } else {
                        match path.last().expect("did we cd ... above root?").get(&name) {
                            Some(node) => {
                                match node.flavor {
                                    DIR => path.push(*node),
                                    FILE => panic!("there was a file i nthe path!"),
                                }
                            },
                            None => panic!("path is empty! did we cd off the top of root?"),
                        }
                    }
                }
            }
            Log::LS => (),
            Log::FILE(name, size) => {
                path.last().expect("did we cd ... above root?").nodes.push(Node::make_file(&name, size));
            }
            Log::DIR(name) => {
                path.last().expect("did we cd ... above root?").nodes.push(Node::make_dir(&name));
            }
        }
    }
    root
}

fn main() {
    let mut f = File::open("input/006.txt").expect("File Error");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("File Read Error");
    
    let filesystem = parse_logs(&input);
    println!("fs: {}", filesystem.name());
}

#[cfg(test)]
mod tests {
    use crate::*;
    const SAMPLE: &str = r#"$ cd /
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
7214296 k"#;

    #[test]
    fn test_parse_log() {
        let filesystem = parse_logs(SAMPLE);
        assert_eq!(filesystem.len(), 4);
        filesystem.get("a").unwrap();
        filesystem.get("b.txt").unwrap();
        filesystem.get("c.dat").unwrap();
        filesystem.get("a").unwrap().get("e").unwrap();
        filesystem.get("a").unwrap().get("e").unwrap().get("i");
    }

    #[test]
    fn test_size() {
        let filesystem = parse_logs(SAMPLE);
        assert_eq!(filesystem.get("a").unwrap().get("e").unwrap().size(), 584);
        assert_eq!(filesystem.get("a").unwrap().size(), 94853);
        assert_eq!(filesystem.get("d").unwrap().size(), 24933642);
        assert_eq!(filesystem.size(), 48381165);
    }

    #[test]
    fn test_cd_regex() {
        assert_eq!(CD_RE.is_match("$ cd /"), true);
        assert_eq!(CD_RE.is_match("$ ls"), false);
        assert_eq!(CD_RE.is_match("dir a"), false);
        assert_eq!(CD_RE.is_match("14848514 b.txt"), false);
        assert_eq!(CD_RE.is_match("dir def"), false);
        assert_eq!(CD_RE.captures("$ cd def").unwrap().get(1).unwrap().as_str(), "def");
    }

    #[test]
    fn test_ls_regex() {
        assert_eq!(LS_RE.is_match("$ cd /"), false);
        assert_eq!(LS_RE.is_match("$ ls"), true);
        assert_eq!(LS_RE.is_match("dir a"), false);
        assert_eq!(LS_RE.is_match("14848514 b.txt"), false);
    }

    #[test]
    fn test_dir_regex() {
        assert_eq!(DIR_RE.is_match("$ cd /"), false);
        assert_eq!(DIR_RE.is_match("$ ls"), false);
        assert_eq!(DIR_RE.is_match("dir a"), true);
        assert_eq!(DIR_RE.is_match("14848514 b.txt"), false);
        assert_eq!(DIR_RE.captures("dir abc").unwrap().get(1).unwrap().as_str(), "abc");
    }

    #[test]
    fn test_file_regex() {
        assert_eq!(FILE_RE.is_match("$ cd /"), false);
        assert_eq!(FILE_RE.is_match("$ ls"), false);
        assert_eq!(FILE_RE.is_match("dir a"), false);
        assert_eq!(FILE_RE.is_match("14848514 b.txt"), true);
        assert_eq!(FILE_RE.captures("14848514 b.txt").unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap(), 14848514);
        assert_eq!(FILE_RE.captures("14848514 b.txt").unwrap().get(2).unwrap().as_str(), "b.txt");
    }

    #[test]
    fn test_parse_log_line() {
        assert_eq!(parse_log_line("$ cd /"), Log::CD(String::from("/")));
        assert_eq!(parse_log_line("$ ls"), Log::LS);
        assert_eq!(parse_log_line("dir a"), Log::DIR(String::from("a")));
        assert_eq!(parse_log_line("14848514 b.txt"), Log::FILE(String::from("b.txt"), 14848514));
    }
}