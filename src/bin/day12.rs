use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

const INPUT: &str = include_str!("../../inputs/day12.txt");

fn main() {
    let cave_map: CaveMap = parse_input(INPUT);
    println!("Answer 1: {}", cave_map.num_paths(VisitMode::Once));
    println!("Answer 2: {}", cave_map.num_paths(VisitMode::SingleTwice));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Cave {
    Start,
    End,
    Big(&'static str),
    Small(&'static str),
}

impl Cave {
    fn new(s: &'static str) -> Self {
        if s == "start" {
            Cave::Start
        } else if s == "end" {
            Cave::End
        } else if s.chars().all(char::is_uppercase) {
            Cave::Big(s)
        } else if s.chars().all(char::is_lowercase) {
            Cave::Small(s)
        } else {
            unreachable!()
        }
    }
}

fn parse_input(input: &'static str) -> CaveMap {
    let mut connections = HashMap::<Cave, HashSet<Cave>>::new();

    for line in input.lines() {
        let (cave1, cave2) = line.split_once('-').unwrap();
        let (cave1, cave2): (Cave, Cave) = (Cave::new(cave1), Cave::new(cave2));

        if let Some(connections_cave1) = connections.get_mut(&cave1) {
            connections_cave1.insert(cave2.clone());
        } else {
            connections.insert(cave1.clone(), HashSet::from([cave2.clone()]));
        }

        if let Some(connections_cave2) = connections.get_mut(&cave2) {
            connections_cave2.insert(cave1.clone());
        } else {
            connections.insert(cave2, HashSet::from([cave1]));
        }
    }

    CaveMap { connections }
}

#[derive(Debug)]
struct CaveMap {
    connections: HashMap<Cave, HashSet<Cave>>,
}

impl CaveMap {
    fn num_paths(&self, mode: VisitMode) -> usize {
        let mut count = 0;
        let mut paths = VecDeque::from([Path::new(mode)]);

        while !paths.is_empty() {
            let path = paths.pop_front().unwrap();
            let last_cave = path.last_cave();

            let cave_connections = self.connections.get(last_cave).unwrap();
            if cave_connections.iter().any(|c| matches!(c, Cave::End)) {
                count += 1;
            }

            let pushable = cave_connections
                .iter()
                .filter(|c| path.can_visit(c))
                .cloned()
                .collect::<Vec<_>>();

            for (path, next) in itertools::repeat_n(path, pushable.len()).zip(pushable.into_iter())
            {
                paths.push_back(Path::from(path, next));
            }
        }

        count
    }
}

#[derive(Debug, Clone)]
struct Path {
    last: Cave,
    visited: HashMap<Cave, usize>,
    single_cave_was_visited_twice: bool,
    mode: VisitMode,
}

#[derive(Debug, PartialEq, Clone)]
enum VisitMode {
    Once,
    SingleTwice,
}

impl Path {
    fn new(mode: VisitMode) -> Self {
        let start = Cave::Start;
        Self {
            last: start.clone(),
            visited: HashMap::from([(start, 1)]),
            single_cave_was_visited_twice: false,
            mode,
        }
    }

    #[inline]
    fn from(mut path: Path, next: Cave) -> Self {
        path.add_cave(next);
        path
    }

    fn add_cave(&mut self, cave: Cave) {
        self.last = cave.clone();

        match self.mode {
            VisitMode::Once => {
                assert!(matches!(cave, Cave::Big(_)) || !self.visited.contains_key(&cave));
                self.visited.insert(cave, 1);
            }
            VisitMode::SingleTwice => {
                if let Some(num_visits) = self.visited.get_mut(&cave) {
                    assert!(matches!(cave, Cave::Big(_)) || num_visits < &mut 2);
                    *num_visits += 1;
                    if matches!(cave, Cave::Small(_)) {
                        self.single_cave_was_visited_twice = true;
                    }
                } else {
                    self.visited.insert(cave, 1);
                }
            }
        }
    }

    fn can_visit(&self, cave: &Cave) -> bool {
        match cave {
            Cave::Start | Cave::End => false,
            Cave::Big(_) => true,
            Cave::Small(_) => match self.mode {
                VisitMode::Once => !self.visited.contains_key(cave),
                VisitMode::SingleTwice => {
                    if self.single_cave_was_visited_twice {
                        !self.visited.contains_key(cave)
                    } else {
                        self.visited.get(cave).unwrap_or(&0) < &2
                    }
                }
            },
        }
    }

    fn last_cave(&self) -> &Cave {
        &self.last
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EXAMPLE1: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end
";

    const INPUT_EXAMPLE2: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
";

    const INPUT_EXAMPLE3: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
";

    #[test]
    fn example() {
        // Part 1
        let cave_map: CaveMap = parse_input(INPUT_EXAMPLE1);
        assert_eq!(cave_map.num_paths(VisitMode::Once), 10);

        let cave_map: CaveMap = parse_input(INPUT_EXAMPLE2);
        assert_eq!(cave_map.num_paths(VisitMode::Once), 19);

        let cave_map: CaveMap = parse_input(INPUT_EXAMPLE3);
        assert_eq!(cave_map.num_paths(VisitMode::Once), 226);

        // Part 2
        let cave_map: CaveMap = parse_input(INPUT_EXAMPLE1);
        assert_eq!(cave_map.num_paths(VisitMode::SingleTwice), 36);

        let cave_map: CaveMap = parse_input(INPUT_EXAMPLE2);
        assert_eq!(cave_map.num_paths(VisitMode::SingleTwice), 103);

        let cave_map: CaveMap = parse_input(INPUT_EXAMPLE3);
        assert_eq!(cave_map.num_paths(VisitMode::SingleTwice), 3509);
    }
}
