#![feature(hash_drain_filter)]
use std::collections::HashSet;

const INPUT: &str = include_str!("../../inputs/day13_hard.txt");

fn main() {
    let (mut dots, folds) = parse_input(INPUT);

    fold(&mut dots, &folds[0]);
    println!("Answer 1: {}", dots.len());

    for f in folds.iter().skip(1) {
        fold(&mut dots, f);
    }
    println!("Answer 2:");
    plot(&dots);
}

#[derive(Debug)]

enum Fold {
    X(usize),
    Y(usize),
}

fn parse_input(input: &str) -> (HashSet<(usize, usize)>, Vec<Fold>) {
    let (dot_lines, fold_lines) = input.split_once("\n\n").unwrap();

    let dots = HashSet::from_iter(dot_lines.lines().map(|line| {
        let (x, y) = line.split_once(',').unwrap();
        (x.parse().unwrap(), y.parse().unwrap())
    }));

    let folds = Vec::from_iter(fold_lines.lines().map(|line| {
        let fold = line.split(' ').nth(2).unwrap();
        let (axis, value) = fold.split_once('=').unwrap();
        let value = value.parse().unwrap();
        match axis {
            "x" => Fold::X(value),
            "y" => Fold::Y(value),
            _ => unreachable!(),
        }
    }));

    (dots, folds)
}

fn fold(dots: &mut HashSet<(usize, usize)>, fold: &Fold) {
    match fold {
        Fold::X(xf) => {
            let foldable: HashSet<_> = dots.drain_filter(|(x, _)| x > xf).collect();
            for (x, y) in foldable {
                if let Some(x) = (2 * xf).checked_sub(x) {
                    dots.insert((x, y));
                }
            }
        }
        Fold::Y(yf) => {
            let foldable: HashSet<_> = dots.drain_filter(|(_, y)| y > yf).collect();
            for (x, y) in foldable {
                if let Some(y) = (2 * yf).checked_sub(y) {
                    dots.insert((x, y));
                }
            }
        }
    }
}

fn plot(dots: &HashSet<(usize, usize)>) {
    let max_x = dots.iter().max_by_key(|(x, _)| x).unwrap().0;
    let max_y = dots.iter().max_by_key(|(_, y)| y).unwrap().1;

    for y in 0..(max_y + 1) {
        for x in 0..(max_x + 1) {
            if dots.contains(&(x, y)) {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EXAMPLE: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";

    #[test]
    fn example() {
        let (mut dots, folds) = parse_input(INPUT_EXAMPLE);

        fold(&mut dots, &folds[0]);
        assert_eq!(dots.len(), 17);

        // Part 2
        fold(&mut dots, &folds[1]);
        plot(&dots);
    }
}
