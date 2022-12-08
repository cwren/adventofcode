use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, hash::Hash, hash::Hasher, io::Read};

#[derive(Clone, Debug, PartialEq)]
enum NodeFlavor {
    Dir,
    File,
}
use crate::NodeFlavor::{Dir, File};

#[derive(Clone, Debug)] 
struct Node {
    flavor: NodeFlavor,
    name: String,
    size: usize,
    children: Vec<String>,
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Node {}

type Filesystem = HashMap<String, Node>;

impl Node {
    fn size(&self, fs: &Filesystem) -> usize {
        match self.flavor {
            NodeFlavor::File => self.size,
            NodeFlavor::Dir => self.children.iter().map(|n| fs.get(n).expect("missing node!").size(fs)).sum(),
        }
    }

    fn make_file(name: &str, size: usize) -> Node {
        Node {flavor: File, name: name.to_string(), size, children: Vec::new()}
    }

    fn make_dir(name: &str) -> Node {
        Node {flavor: Dir, name: name.to_string(), size: 0, children: Vec::new()}
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
    static ref CD_RE: regex::Regex = Regex::new(r"^\$ cd ([/a-zA-Z0-9.]+)$").unwrap();
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

fn up_from(cwd :&str) -> String {
    if let Some(i) = cwd.rfind('/') {
        let (a, _) = cwd.split_at(i);
        if !a.is_empty() {
            return a.to_string();
        } else {
            return String::from("/");
        }
    }
    String::new()
}

fn make_name(cwd :&str, node: &str) -> String {
    match cwd.len() {
        0 => node.to_string(),
        1 => [cwd, node].join(""),
        _ => [cwd, node].join("/")
    }
}

fn parse_logs(input: &str) -> Filesystem {
    let mut fs = Filesystem::new();
    fs.insert(String::from("/"), Node::make_dir("/")); //root directory

    let mut cwd = String::new();
    for line in input.lines() {
        match parse_log_line(line) {
            Log::CD(name) => {
                if name == ".." {
                    cwd = up_from(&cwd);
                } else { 
                    cwd = make_name(&cwd, &name);
                }
                assert!(fs.contains_key(&cwd));
            }
            Log::LS => (),
            Log::FILE(name, size) => {
                let filename  = make_name(&cwd, &name);
                fs.insert(filename.clone(), Node::make_file(&filename, size));
                fs.get_mut(&cwd).expect("adding to unknwon directory").children.push(filename.clone());
            }
            Log::DIR(name) => {
                let dirname = make_name(&cwd, &name);
                fs.insert(dirname.clone(), Node::make_dir(&dirname));
                fs.get_mut(&cwd).expect("adding to unknwon directory").children.push(dirname.clone());
            }
        }
    }
    fs
}

fn part_1(fs: &HashMap<String, Node>) -> usize{
    fs.iter()
        .filter(|n| n.1.flavor == NodeFlavor::Dir && n.1.size(fs) <= 100000)
        .map(|n| n.1.size(fs)).sum()
}

fn part_2(fs: &HashMap<String, Node>) -> usize{
    let used = fs.get("/").expect("root directory missing").size(fs);
    let free = 70000000 - used;
    let goal = 30000000 - free;
    assert!(goal > 0);
    let mut candidates = fs.iter()
        .filter(|n| n.1.flavor == NodeFlavor::Dir && n.1.size(fs) > goal)
        .map(|n| n.1)
        .collect::<Vec<&Node>>();
    candidates.sort_by(|a, b| a.size(fs).partial_cmp(&b.size(fs)).unwrap());
    candidates.first().expect("no candidates found").size(fs)
}

fn main() {
    let mut f = std::fs::File::open("input/007.txt").expect("File Error");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("File Read Error");
    
    let fs = parse_logs(&input);
    let t = part_1(&fs);
    println!("total of dir larger than 100000 is {t}");
    let f = part_2(&fs);
    println!("size of directory to be freed is {f}");
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
// - / (dir)
//   - a (dir)
//     - e (dir)
//       - i (file, size=584)
//     - f (file, size=29116)
//     - g (file, size=2557)
//     - h.lst (file, size=62596)
//   - b.txt (file, size=14848514)
//   - c.dat (file, size=8504156)
//   - d (dir)
//     - j (file, size=4060174)
//     - d.log (file, size=8033020)
//     - d.ext (file, size=5626152)
//     - k (file, size=7214296)
    #[test]
    fn test_parse_log() {
        let filesystem = parse_logs(SAMPLE);
        assert_eq!(filesystem.len(), 14);
        println!("{:?}", filesystem);
        filesystem.get("/a").unwrap();
        filesystem.get("/b.txt").unwrap();
        filesystem.get("/c.dat").unwrap();
        filesystem.get("/a/e").unwrap();
        filesystem.get("/a/e/i").unwrap();
    }

    #[test]
    fn test_size() {
        let filesystem = parse_logs(SAMPLE);
        assert_eq!(filesystem.get("/a/e").unwrap().size(&filesystem), 584);
        assert_eq!(filesystem.get("/a").unwrap().size(&filesystem), 94853);
        assert_eq!(filesystem.get("/d").unwrap().size(&filesystem), 24933642);
        assert_eq!(filesystem.get("/").unwrap().size(&filesystem), 48381165);
    }

    #[test]
    fn test_cd_regex() {
        assert_eq!(CD_RE.is_match("$ cd /"), true);
        assert_eq!(CD_RE.is_match("$ cd .."), true);
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

    #[test]
    fn test_up_from() {
        assert_eq!(up_from("/a/e/d"), "/a/e");
        assert_eq!(up_from("/a/e"), "/a");
        assert_eq!(up_from("/a"), "/");
    }

    #[test]
    fn test_part_1() {
        let fs = parse_logs(SAMPLE);
        assert_eq!(part_1(&fs), 95437);
    }

    #[test]
    fn test_part_2() {
        let fs = parse_logs(SAMPLE);
        assert_eq!(part_2(&fs), 24933642);
    }

}