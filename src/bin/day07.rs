#![feature(int_abs_diff)]

const INPUT: &str = include_str!("../../inputs/day07.txt");

fn main() {
    let positions = parse_input(INPUT);

    println!("Answer 1: {}", mininum_fuel(&positions, cost1));
    println!("Answer 2: {}", mininum_fuel(&positions, cost2));
}

fn parse_input(input: &str) -> Vec<u16> {
    input
        .trim()
        .split(',')
        .map(|e| e.parse().unwrap())
        .collect()
}

fn mininum_fuel<F>(positions: &[u16], cost_fn: F) -> u32
where
    F: Fn(u16, u16) -> u32,
{
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    (min..=max)
        .map(|pos| positions.iter().map(|p| cost_fn(*p, pos)).sum())
        .min()
        .unwrap()
}

#[inline]
fn cost1(p1: u16, p2: u16) -> u32 {
    p1.abs_diff(p2) as u32
}

#[inline]
fn cost2(p1: u16, p2: u16) -> u32 {
    let d = cost1(p1, p2);
    ((1 + d) * d) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EXAMPLE: &str = "16,1,2,0,4,2,7,1,2,14
";

    #[test]
    fn example() {
        let positions = parse_input(INPUT_EXAMPLE);
        assert_eq!(positions, [16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);

        // Part 1
        assert_eq!(mininum_fuel(&positions, cost1), 37);

        // Part 2
        assert_eq!(mininum_fuel(&positions, cost2), 168);
    }
}
