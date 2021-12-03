const INPUT: &str = include_str!("../../inputs/day03.txt");

fn main() {
    let report = parse_input(INPUT);
    let (gamma_rate, epsilon_rate) = calculate_gamma_and_epsilon(&report);
    println!(
        "gamma_rate = {}, epsilon_rate = {}, answer 1 = {}",
        gamma_rate,
        epsilon_rate,
        gamma_rate * epsilon_rate
    );

    let oxygen_rate = calculate_oxygen_rate(&report);
    let co2_rate = calculate_co2_rate(&report);
    println!(
        "oxygen_rate = {}, co2_rate = {}, answer 2 = {}",
        oxygen_rate,
        co2_rate,
        oxygen_rate * co2_rate
    )
}

#[derive(Debug, Clone)]
struct BinaryVec(Vec<bool>);

impl std::str::FromStr for BinaryVec {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(v) = s
            .chars()
            .map(|c| match c {
                '0' => Some(false),
                '1' => Some(true),
                _ => None,
            })
            .collect::<Option<Vec<_>>>()
        {
            return Ok(BinaryVec(v));
        }
        Err(())
    }
}

impl<const N: usize> PartialEq<[bool; N]> for BinaryVec {
    fn eq(&self, other: &[bool; N]) -> bool {
        self.0 == other
    }
}

fn parse_input(input: &str) -> Vec<BinaryVec> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn most_common_value_in_position(report: &[BinaryVec], i: usize) -> Option<bool> {
    let mut count = 0;
    for BinaryVec(r) in report {
        match r[i] {
            true => count += 1,
            false => count -= 1,
        }
    }
    let result = if count == 0 { None } else { Some(count > 0) };
    result
}

fn calculate_gamma_and_epsilon(report: &[BinaryVec]) -> (usize, usize) {
    assert!(report.len() != 0);
    let num_bits = report[0].0.len();
    let mut gamma_rate = vec![false; num_bits];

    for i in 0..num_bits {
        gamma_rate[i] = most_common_value_in_position(report, i).unwrap();
    }

    let epsilon_rate = BinaryVec(gamma_rate.iter().map(|bit| !bit).collect());
    let gamma_rate = BinaryVec(gamma_rate);

    let gamma_rate: usize = gamma_rate.into();
    let epsilon_rate: usize = epsilon_rate.into();

    (gamma_rate, epsilon_rate)
}

impl From<BinaryVec> for usize {
    fn from(v: BinaryVec) -> Self {
        let mut value = 0;
        let num_bits = v.0.len();
        let mut power = 0;
        for i in (0..num_bits).rev() {
            if v.0[i] {
                value += usize::pow(2, power);
            }

            power += 1;
        }
        value
    }
}

fn calculate_oxygen_rate(report: &[BinaryVec]) -> usize {
    let mut candidates = Vec::from(report);
    let mut bit_position = 0;
    while candidates.len() > 1 {
        let most_common = most_common_value_in_position(&candidates, bit_position).unwrap_or(true);
        candidates = candidates
            .into_iter()
            .filter(|c| c.0[bit_position] == most_common)
            .collect();
        bit_position += 1;
    }
    candidates[0].clone().into()
}

fn calculate_co2_rate(report: &[BinaryVec]) -> usize {
    let mut candidates = Vec::from(report);
    let mut bit_position = 0;
    while candidates.len() > 1 {
        let least_common =
            !most_common_value_in_position(&candidates, bit_position).unwrap_or(true);
        candidates = candidates
            .into_iter()
            .filter(|c| c.0[bit_position] == least_common)
            .collect();
        bit_position += 1;
    }
    candidates[0].clone().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EXAMPLE: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

    #[test]
    fn example() {
        let report = parse_input(INPUT_EXAMPLE);
        assert_eq!(
            report,
            [
                [false, false, true, false, false],
                [true, true, true, true, false],
                [true, false, true, true, false],
                [true, false, true, true, true],
                [true, false, true, false, true],
                [false, true, true, true, true],
                [false, false, true, true, true],
                [true, true, true, false, false],
                [true, false, false, false, false],
                [true, true, false, false, true],
                [false, false, false, true, false],
                [false, true, false, true, false],
            ]
        );

        let (gamma_rate, epsilon_rate) = calculate_gamma_and_epsilon(&report);
        assert_eq!(gamma_rate, 22);
        assert_eq!(epsilon_rate, 9);
        assert_eq!(gamma_rate * epsilon_rate, 198);

        let oxygen_rate = calculate_oxygen_rate(&report);
        let co2_rate = calculate_co2_rate(&report);
        assert_eq!(oxygen_rate, 23);
        assert_eq!(co2_rate, 10);
        assert_eq!(oxygen_rate * co2_rate, 230);
    }
}
