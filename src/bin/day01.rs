#![feature(array_windows)]

const INPUT: &str = include_str!("../../inputs/day01.txt");

fn main() {
    let depths = parse_input(INPUT);

    let answer = count_increases(&depths);
    println!("Number of increases: {}", answer);

    let answer = count_increases_three_measurements_window(&depths);
    println!(
        "Number of increases with three measurements window: {}",
        answer
    );
}

fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn count_increases(depths: &[usize]) -> usize {
    depths.array_windows().filter(|[d1, d2]| d2 > d1).count()
}

fn count_increases_three_measurements_window(depths: &[usize]) -> usize {
    depths
        .array_windows()
        .map(|[d1, d2, d3]| d1 + d2 + d3)
        .collect::<Vec<_>>() // FIXME: can we skip collecting here?
        .array_windows()
        .filter(|[s1, s2]| s2 > s1)
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_EXAMPLE: &str = "199
200
208
210
200
207
240
269
260
263
";

    #[test]
    fn example() {
        let depths = parse_input(INPUT_EXAMPLE);

        assert_eq!(depths, [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
        // Part 1
        assert_eq!(count_increases(&depths), 7);

        // Part 2
        assert_eq!(count_increases_three_measurements_window(&depths), 5);
    }
}
