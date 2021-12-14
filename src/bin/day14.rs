#![feature(array_windows)]

use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/day14.txt");

fn main() {
    let (polymer_template, pair_insertion_rules) = parse_input(INPUT);
    println!(
        "Answer 1: {}",
        diff_after_steps(&polymer_template, &pair_insertion_rules, 10)
    );
    println!(
        "Answer 2: {}",
        diff_after_steps(&polymer_template, &pair_insertion_rules, 40)
    );
}

type Polymer = Vec<char>;
type PairInsertionRules = HashMap<([char; 2]), char>;

fn parse_input(s: &str) -> (Polymer, PairInsertionRules) {
    let (polymer_template, pair_insertion_rules) = s.split_once("\n\n").unwrap();
    let pair_insertion_rules = pair_insertion_rules
        .lines()
        .map(|line| {
            let (pair, to_be_inserted) = line.split_once(" -> ").unwrap();
            (
                pair.chars().collect::<Vec<_>>().try_into().unwrap(),
                to_be_inserted.chars().next().unwrap(),
            )
        })
        .collect();

    (polymer_template.chars().collect(), pair_insertion_rules)
}

fn diff_after_steps(
    polymer: &Polymer,
    pair_insertion_rules: &PairInsertionRules,
    steps: usize,
) -> usize {
    let mut count_pairs = polymer
        .array_windows()
        .map(|[a, b]| ([*a, *b], 1usize))
        .collect::<HashMap<_, _>>();
    let last_char = *polymer.last().unwrap();

    for _ in 0..steps {
        let mut new_count_pairs = HashMap::new();
        for ([a, b], count) in count_pairs {
            let new_char = *pair_insertion_rules.get(&[a, b]).unwrap();
            *new_count_pairs.entry([a, new_char]).or_default() += count;
            *new_count_pairs.entry([new_char, b]).or_default() += count;
        }
        count_pairs = new_count_pairs;
    }

    let mut counts =
        count_pairs
            .into_iter()
            .fold(HashMap::<_, usize>::new(), |mut acc, ([a, _], count)| {
                *acc.entry(a).or_default() += count;
                acc
            });
    *counts.entry(last_char).or_default() += 1;
    let counts = counts.into_values().collect::<Vec<_>>();

    counts.iter().max().unwrap() - counts.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EXAMPLE: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";

    #[test]
    fn part1() {
        let (polymer_template, pair_insertion_rules) = parse_input(INPUT_EXAMPLE);
        assert_eq!(polymer_template, ['N', 'N', 'C', 'B']);
        assert_eq!(
            pair_insertion_rules,
            HashMap::from([
                (['C', 'H'], 'B'),
                (['H', 'H'], 'N'),
                (['C', 'B'], 'H'),
                (['N', 'H'], 'C'),
                (['H', 'B'], 'C'),
                (['H', 'C'], 'B'),
                (['H', 'N'], 'C'),
                (['N', 'N'], 'C'),
                (['B', 'H'], 'H'),
                (['N', 'C'], 'B'),
                (['N', 'B'], 'B'),
                (['B', 'N'], 'B'),
                (['B', 'B'], 'N'),
                (['B', 'C'], 'B'),
                (['C', 'C'], 'N'),
                (['C', 'N'], 'C'),
            ])
        );

        assert_eq!(
            diff_after_steps(&polymer_template, &pair_insertion_rules, 10),
            1588
        );
        assert_eq!(
            diff_after_steps(&polymer_template, &pair_insertion_rules, 40),
            2188189693529
        );
    }
}
