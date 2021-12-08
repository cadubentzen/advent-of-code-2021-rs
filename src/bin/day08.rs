use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../inputs/day08.txt");

fn main() {
    let mut entries = parse_input(INPUT);

    println!("Answer 1: {}", count_1_4_7_in_output(&entries));
    println!("Answer 2: {}", add_decoded_outputs(&mut entries));
}

#[derive(Debug, PartialEq, Clone)]
struct Entry {
    signal_patterns: [Vec<char>; 10],
    output: [Vec<char>; 4],
}

impl Entry {
    fn normalize(&mut self) {
        for sp in &mut self.signal_patterns {
            sp.sort_unstable();
        }
        for o in &mut self.output {
            o.sort_unstable();
        }
    }

    fn decode_output(&mut self) -> u16 {
        self.normalize();

        let mut display = Display::default();

        for n in [1, 7, 4, 8] {
            let code = self
                .signal_patterns
                .iter()
                .find(|p| p.len() == segments_for(n).len())
                .unwrap();
            display.set_code(n, code);
        }

        for n in [9, 6, 0] {
            let code = display
                .get_candidates(n)
                .iter()
                .find(|c| self.signal_patterns.contains(c))
                .cloned()
                .unwrap();
            display.set_code(n, &code);
        }

        assert!(display.is_solved());

        self.output
            .iter()
            .map(|o| display.decode(o))
            .fold(0, |acc, n| 10 * acc + n as u16)
    }
}

impl std::str::FromStr for Entry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (signal_patterns, output) = s.split_once::<'_>('|').unwrap();
        let signal_patterns = signal_patterns
            .trim()
            .split_whitespace()
            .map(|e| e.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let output = output
            .trim()
            .split_whitespace()
            .map(|e| e.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Ok(Self {
            signal_patterns,
            output,
        })
    }
}

fn parse_input(input: &str) -> Vec<Entry> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn count_1_4_7_in_output(entries: &[Entry]) -> usize {
    entries
        .iter()
        .map(|e| &e.output)
        .map(|outs| {
            outs.iter()
                .filter(|o| matches!(o.len(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum()
}

#[derive(Debug, Default)]
struct Display {
    segment_candidates: [HashSet<char>; 7],
    taken: HashSet<char>,
    lookup: HashMap<char, u8>,
}

fn segments_for(n: u8) -> &'static [u8] {
    match n {
        0 => &[0, 1, 2, 3, 4, 5],
        1 => &[1, 2],
        2 => &[0, 1, 3, 4, 6],
        3 => &[0, 1, 2, 3, 6],
        4 => &[1, 2, 5, 6],
        5 => &[0, 2, 3, 5, 6],
        6 => &[0, 2, 3, 4, 5, 6],
        7 => &[0, 1, 2],
        8 => &[0, 1, 2, 3, 4, 5, 6],
        9 => &[0, 1, 2, 3, 5, 6],
        _ => unreachable!(),
    }
}

fn number_from_segments(segments: &[u8]) -> u8 {
    match segments {
        [0, 1, 2, 3, 4, 5] => 0,
        [1, 2] => 1,
        [0, 1, 3, 4, 6] => 2,
        [0, 1, 2, 3, 6] => 3,
        [1, 2, 5, 6] => 4,
        [0, 2, 3, 5, 6] => 5,
        [0, 2, 3, 4, 5, 6] => 6,
        [0, 1, 2] => 7,
        [0, 1, 2, 3, 4, 5, 6] => 8,
        [0, 1, 2, 3, 5, 6] => 9,
        _ => unreachable!(),
    }
}

impl Display {
    fn set_code(&mut self, num: u8, code: &[char]) {
        let segments = segments_for(num);

        let mut solve_for = |seg1: usize, seg2: usize| {
            self.segment_candidates[seg1].retain(|c| code.contains(c));
            let solved = self.segment_candidates[seg1]
                .iter()
                .cloned()
                .next()
                .unwrap();
            self.lookup.insert(solved, seg1 as u8);
            self.segment_candidates[seg2].remove(&solved);
            let solved = self.segment_candidates[seg2]
                .iter()
                .cloned()
                .next()
                .unwrap();
            self.lookup.insert(solved, seg2 as u8);
        };

        match num {
            1 | 4 | 7 | 8 => {
                let not_taken = code
                    .iter()
                    .filter(|c| !self.taken.contains(c))
                    .cloned()
                    .collect::<Vec<_>>();
                for s in segments {
                    let candidates = &mut self.segment_candidates[*s as usize];
                    if candidates.is_empty() {
                        for c in &not_taken {
                            candidates.insert(*c);
                        }
                        for c in code {
                            self.taken.insert(*c);
                        }
                        if num == 7 {
                            self.lookup.insert(*not_taken.first().unwrap(), *s);
                        }
                    }
                }
            }
            9 => solve_for(3, 4),
            6 => solve_for(2, 1),
            0 => {
                solve_for(5, 6);
                assert!(self.lookup.len() == 7);
            }
            _ => unreachable!(),
        }
    }

    fn get_candidates(&self, n: u8) -> Vec<Vec<char>> {
        let expand_candidates = |solved_indices: &[usize], expandable_index: usize| {
            let mut candidates = vec![];
            let base = solved_indices
                .iter()
                .flat_map(|i| self.segment_candidates[*i].iter().cloned())
                .collect::<Vec<_>>();

            for c in &self.segment_candidates[expandable_index] {
                let mut base = base.clone();
                base.push(*c);
                base.sort_unstable();
                candidates.push(base);
            }
            candidates
        };

        // Handcrafted logic here to make expanding combinations easier
        match n {
            9 => {
                assert!(
                    self.segment_candidates[0].len() == 1
                        && self.segment_candidates[1].len() == 2
                        && self.segment_candidates[2].len() == 2
                        && self.segment_candidates[5].len() == 2
                        && self.segment_candidates[6].len() == 2
                        && self.segment_candidates[3].len() == 2,
                    "Segments should have only these numbers of candidates by now"
                );

                expand_candidates(&[0, 1, 5], 3)
            }
            6 => {
                assert!(
                    self.segment_candidates[0].len() == 1
                        && self.segment_candidates[2].len() == 2
                        && self.segment_candidates[3].len() == 1
                        && self.segment_candidates[4].len() == 1
                        && self.segment_candidates[5].len() == 2
                        && self.segment_candidates[6].len() == 2,
                    "Segments should have only these numbers of candidates by now"
                );

                expand_candidates(&[0, 3, 4, 5], 2)
            }
            0 => {
                assert!(
                    self.segment_candidates[0].len() == 1
                        && self.segment_candidates[1].len() == 1
                        && self.segment_candidates[2].len() == 1
                        && self.segment_candidates[3].len() == 1
                        && self.segment_candidates[4].len() == 1
                        && self.segment_candidates[5].len() == 2
                        && self.segment_candidates[6].len() == 2,
                    "Segments should have only these numbers of candidates by now"
                );

                expand_candidates(&[0, 1, 2, 3, 4], 5)
            }
            _ => unreachable!(),
        }
    }

    fn is_solved(&self) -> bool {
        self.segment_candidates.iter().all(|c| c.len() == 1)
    }

    fn decode(&self, pattern: &[char]) -> u8 {
        assert!(self.is_solved());
        let mut segments = pattern
            .iter()
            .map(|c| self.lookup.get(c).unwrap())
            .cloned()
            .collect::<Vec<_>>();
        segments.sort_unstable();
        number_from_segments(&segments)
    }
}

fn add_decoded_outputs(entries: &mut [Entry]) -> u32 {
    entries.iter_mut().map(|e| e.decode_output() as u32).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EXAMPLE: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";

    #[test]
    fn example() {
        let mut entries = parse_input(INPUT_EXAMPLE);
        assert_eq!(
            entries[0],
            Entry {
                signal_patterns: [
                    "be".chars().collect(),
                    "cfbegad".chars().collect(),
                    "cbdgef".chars().collect(),
                    "fgaecd".chars().collect(),
                    "cgeb".chars().collect(),
                    "fdcge".chars().collect(),
                    "agebfd".chars().collect(),
                    "fecdb".chars().collect(),
                    "fabcd".chars().collect(),
                    "edb".chars().collect()
                ],
                output: [
                    "fdgacbe".chars().collect(),
                    "cefdb".chars().collect(),
                    "cefbgd".chars().collect(),
                    "gcbe".chars().collect()
                ]
            }
        );
        assert_eq!(
            entries.last().unwrap(),
            &Entry {
                signal_patterns: [
                    "gcafb".chars().collect(),
                    "gcf".chars().collect(),
                    "dcaebfg".chars().collect(),
                    "ecagb".chars().collect(),
                    "gf".chars().collect(),
                    "abcdeg".chars().collect(),
                    "gaef".chars().collect(),
                    "cafbge".chars().collect(),
                    "fdbac".chars().collect(),
                    "fegbdc".chars().collect()
                ],
                output: [
                    "fgae".chars().collect(),
                    "cfgab".chars().collect(),
                    "fg".chars().collect(),
                    "bagce".chars().collect()
                ]
            }
        );

        // Part 1
        assert_eq!(count_1_4_7_in_output(&entries), 26);

        // Part 2
        let mut entry: Entry = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
        cdfeb fcadb cdfeb cdbaf"
            .parse()
            .unwrap();
        entry.normalize();

        assert_eq!(
            entry,
            Entry {
                signal_patterns: [
                    ['a', 'b', 'c', 'd', 'e', 'f', 'g'].into(),
                    ['b', 'c', 'd', 'e', 'f'].into(),
                    ['a', 'c', 'd', 'f', 'g'].into(),
                    ['a', 'b', 'c', 'd', 'f'].into(),
                    ['a', 'b', 'd'].into(),
                    ['a', 'b', 'c', 'd', 'e', 'f'].into(),
                    ['b', 'c', 'd', 'e', 'f', 'g'].into(),
                    ['a', 'b', 'e', 'f'].into(),
                    ['a', 'b', 'c', 'd', 'e', 'g'].into(),
                    ['a', 'b'].into()
                ],
                output: [
                    ['b', 'c', 'd', 'e', 'f'].into(),
                    ['a', 'b', 'c', 'd', 'f'].into(),
                    ['b', 'c', 'd', 'e', 'f'].into(),
                    ['a', 'b', 'c', 'd', 'f'].into()
                ]
            }
        );

        assert_eq!(entry.decode_output(), 5353);

        let decoded_outputs = entries
            .iter_mut()
            .map(|e| e.decode_output())
            .collect::<Vec<_>>();
        assert_eq!(
            decoded_outputs,
            [8394, 9781, 1197, 9361, 4873, 8418, 4548, 1625, 8717, 4315]
        );

        assert_eq!(add_decoded_outputs(&mut entries), 61229);
    }
}
