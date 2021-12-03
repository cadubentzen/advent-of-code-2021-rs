const INPUT: &str = include_str!("../../inputs/day03.txt");

fn main() {
    let report: Report = INPUT.parse().unwrap();
    let (gamma_rate, epsilon_rate) = report.calculate_gamma_and_epsilon();
    println!(
        "gamma_rate = {}, epsilon_rate = {}, answer 1 = {}",
        gamma_rate,
        epsilon_rate,
        gamma_rate * epsilon_rate
    );

    let oxygen_rate = report.calculate_oxygen();
    let co2_rate = report.calculate_co2();
    println!(
        "oxygen_rate = {}, co2_rate = {}, answer 2 = {}",
        oxygen_rate,
        co2_rate,
        oxygen_rate * co2_rate
    )
}

struct Report {
    values: Vec<usize>,
    num_bits: usize,
}

impl std::str::FromStr for Report {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .lines()
            .map(|line| usize::from_str_radix(line, 2).unwrap())
            .collect();

        assert!(s.lines().next().is_some());
        Ok(Report {
            values,
            num_bits: s.lines().next().unwrap().len(),
        })
    }
}

fn most_common_value_in_position(values: &[usize], pos: usize) -> Option<usize> {
    let mut count = 0;
    for v in values {
        match (v >> pos) & 0b1 {
            1 => count += 1,
            0 => count -= 1,
            _ => unreachable!(),
        }
    }

    match count {
        _ if count > 0 => Some(1),
        _ if count < 0 => Some(0),
        _ => None,
    }
}

impl Report {
    fn calculate_gamma_and_epsilon(&self) -> (usize, usize) {
        let mut gamma_rate = 0;
        for i in 0..self.num_bits {
            gamma_rate += most_common_value_in_position(&self.values, i).unwrap() << i;
        }
        let epsilon_rate = !gamma_rate & ((1 << self.num_bits) - 1);

        (gamma_rate, epsilon_rate)
    }

    fn calculate_oxygen(&self) -> usize {
        let mut candidates = self.values.clone();
        let mut bit_position = self.num_bits;
        while candidates.len() > 1 {
            bit_position -= 1;
            let most_common = most_common_value_in_position(&candidates, bit_position).unwrap_or(1);
            candidates = candidates
                .into_iter()
                .filter(|v| ((v >> bit_position) & 0b1) == most_common)
                .collect();
        }
        candidates[0]
    }

    fn calculate_co2(&self) -> usize {
        let mut candidates = self.values.clone();
        let mut bit_position = self.num_bits;
        while candidates.len() > 1 {
            bit_position -= 1;
            let least_common =
                1 - most_common_value_in_position(&candidates, bit_position).unwrap_or(1);
            candidates = candidates
                .into_iter()
                .filter(|v| ((v >> bit_position) & 0b1) == least_common)
                .collect();
        }
        candidates[0]
    }
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
        let report: Report = INPUT_EXAMPLE.parse().unwrap();
        assert_eq!(
            report.values,
            [
                0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
                0b11001, 0b00010, 0b01010,
            ]
        );
        assert_eq!(report.num_bits, 5);

        let (gamma_rate, epsilon_rate) = report.calculate_gamma_and_epsilon();
        assert_eq!(gamma_rate, 22);
        assert_eq!(epsilon_rate, 9);
        assert_eq!(gamma_rate * epsilon_rate, 198);

        let oxygen_rate = report.calculate_oxygen();
        let co2_rate = report.calculate_co2();
        assert_eq!(oxygen_rate, 23);
        assert_eq!(co2_rate, 10);
        assert_eq!(oxygen_rate * co2_rate, 230);
    }
}
