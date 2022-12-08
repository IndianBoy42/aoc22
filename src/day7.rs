use crate::utils::*;

#[derive(Clone, Debug, PartialEq)]
struct Dir {
    name: String,
    size: Option<usize>,
    contains: Vec<usize>,
}

impl Dir {
    fn new(path: String) -> Self {
        Self {
            name: path,
            size: None,
            contains: vec![],
        }
    }
    fn root() -> Self {
        Self {
            name: "/".to_string(),
            size: None,
            contains: vec![],
        }
    }
    fn file(path: String, size: usize) -> Self {
        Self {
            name: path,
            size: Some(size),
            contains: vec![],
        }
    }
    fn calc_size(&self, dirs: &[Self]) -> usize {
        self.size.unwrap_or_else(|| {
            self.contains
                .iter()
                .map(|&i| {
                    let (dir, rest) = dirs.split_at(i + 1);
                    dir.last().unwrap().calc_size(rest)
                })
                .sum()
        })
    }
    fn calc_size_mut(&mut self, dirs: &mut [Self]) -> usize {
        if let Some(size) = self.size {
            size
        } else {
            let size = self
                .contains
                .iter()
                .map(|&i| {
                    let (dir, rest) = dirs.split_at_mut(i + 1);
                    dir.last_mut().unwrap().calc_size_mut(rest)
                })
                .sum();
            self.size = Some(size);
            size
        }
    }
}
/// Example input:
/// $ cd /
/// $ ls
/// dir a
/// 14848514 b.txt
/// 8504156 c.dat
/// dir d
/// $ cd a
/// $ ls
/// dir e
/// 29116 f
/// 2557 g
/// 62596 h.lst
/// $ cd e
/// $ ls
/// 584 i
/// $ cd ..
/// $ cd ..
/// $ cd d
/// $ ls
/// 4060174 j
/// 8033020 d.log
/// 5626152 d.ext
/// 7214296 k
fn parse(input: &str) -> Vec<Dir> {
    let mut lines = input.lines();
    let mut dirstack = vec![0];
    let mut fs = vec![Dir::root()];
    while let Some(line) = lines.next() {
        // println!("line: {line}");
        if !line.starts_with('$') {
            unreachable!("Invalid input");
        }

        if let Some(cdto) = line.strip_prefix("$ cd ") {
            if cdto == ".." {
                dirstack.pop();
            } else if cdto == "/" {
                dirstack.drain(1..);
            } else {
                fs.push(Dir::new(
                    // fs[dirstack.last()].map_or(String::new(), |x| x.path) +
                    cdto.trim().to_string(),
                ));

                let cd = *dirstack.last().unwrap();
                let i = fs.len() - 1 - cd - 1;
                fs[cd].contains.push(i);
                dirstack.push(i + cd + 1);
            }
        } else if line.strip_prefix("$ ls").is_some() {
            let cd = *dirstack.last().unwrap();
            let i = fs.len() - 1 - cd;
            fs.extend(
                lines
                    .take_while_ref(|line| !line.starts_with('$'))
                    .filter(|line| !line.starts_with("dir"))
                    .map(|line| {
                        let mut parts = line.split_whitespace();
                        let (size, name) = (parts.next().unwrap(), parts.next().unwrap());
                        Dir::file(name.to_owned(), size.parse::<usize>().ok().unwrap())
                    }),
            );
            let j = fs.len() - 1 - cd;
            fs[cd].contains.extend(i..j);
        }
        // println!("{dirstack:#?} {fs:#?}");
    }

    let (root, dirs) = fs.split_first_mut().unwrap();
    root.calc_size_mut(dirs);
    // TODO: Iterate backwards through the array filling in size fields
    // fs.iter_mut().enumerate().rev().for_each(|(i, dir)| {
    //     dir.size = Some(dir.size.unwrap_or_else(|| dir.calc_size(&fs[i..])));
    // });

    fs
}

pub fn part1(input: &str) -> usize {
    let dirs = parse(input);

    dirs.iter()
        .filter(|f| !f.contains.is_empty()) // Get Directories
        .filter_map(|d| d.size)
        .filter(|&d| d <= 100_000)
        .sum()
}

pub fn part2(input: &str) -> usize {
    const TOTAL: usize = 70_000_000;
    const UNUSED: usize = 30_000_000;
    let dirs = parse(input);
    println!("{dirs:#?}");
    let unused = TOTAL - dirs[0].size.unwrap();
    dirs.iter()
        .filter(|f| !f.contains.is_empty()) // Get Directories
        .filter_map(|d| d.size)
        .sorted()
        .find(|s| s + unused > UNUSED)
        .unwrap()
}

#[test]
fn test() {
    let input = read_input("input7.txt").unwrap();
    let test = "$ cd /
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
    assert_eq!(part1(&input), 1297159);
    assert_eq!(part2(&input), 3866390);
}
