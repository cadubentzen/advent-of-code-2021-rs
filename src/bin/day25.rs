use std::{fmt::Display, str::FromStr};

const INPUT: &str = include_str!("../../inputs/day25.txt");

fn main() {
    let mut sea_cucumber_map = parse_input(INPUT);
    println!("Answer 1: {}", sea_cucumber_map.step_until_stop());
}

#[derive(Debug, Clone)]
enum Cucumber {
    East,
    South,
}

#[derive(Debug, Clone)]
enum Occupancy {
    Busy(Cucumber),
    Empty,
}

impl FromStr for Occupancy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "v" => Ok(Self::Busy(Cucumber::South)),
            ">" => Ok(Self::Busy(Cucumber::East)),
            "." => Ok(Self::Empty),
            _ => Err(()),
        }
    }
}

impl Display for Occupancy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Occupancy::Busy(Cucumber::East) => ">",
                Occupancy::Busy(Cucumber::South) => "v",
                Occupancy::Empty => ".",
            }
        )
    }
}

#[derive(Debug)]
struct SeaCucumberMap {
    positions: Vec<Vec<Occupancy>>,
}

impl Display for SeaCucumberMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.positions {
            for pos in line {
                write!(f, "{}", pos)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl SeaCucumberMap {
    fn step_until_stop(&mut self) -> usize {
        for i in 1.. {
            if !self.step() {
                return i;
            }
        }
        unreachable!()
    }

    fn step(&mut self) -> bool {
        let mut moved = false;

        let height = self.positions.len();
        let width = self.positions[0].len();

        let mut new_positions = self.positions.clone();

        // Move east-facing
        for (i, line) in self.positions.iter().enumerate() {
            for (j, pos) in line.iter().enumerate() {
                if let Occupancy::Busy(Cucumber::East) = pos {
                    let mut new_j = j + 1;
                    if new_j >= width {
                        new_j = 0;
                    }
                    if let Occupancy::Empty = self.positions[i][new_j] {
                        new_positions[i][new_j] = Occupancy::Busy(Cucumber::East);
                        new_positions[i][j] = Occupancy::Empty;
                        moved = true;
                    }
                }
            }
        }

        self.positions = new_positions.clone();

        // Move south-facing
        for (i, line) in self.positions.iter().enumerate() {
            for (j, pos) in line.iter().enumerate() {
                if let Occupancy::Busy(Cucumber::South) = pos {
                    let mut new_i = i + 1;
                    if new_i >= height {
                        new_i = 0;
                    }
                    if let Occupancy::Empty = self.positions[new_i][j] {
                        new_positions[new_i][j] = Occupancy::Busy(Cucumber::South);
                        new_positions[i][j] = Occupancy::Empty;
                        moved = true;
                    }
                }
            }
        }

        self.positions = new_positions;

        moved
    }
}

fn parse_input(s: &str) -> SeaCucumberMap {
    let positions = s
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Occupancy::from_str(&c.to_string()).unwrap())
                .collect()
        })
        .collect();
    SeaCucumberMap { positions }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EXAMPLE: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>
";

    #[test]
    fn example() {
        let mut sea_cucumber_map = parse_input(INPUT_EXAMPLE);
        assert_eq!(sea_cucumber_map.step_until_stop(), 58);
        assert_eq!(
            sea_cucumber_map.to_string(),
            "..>>v>vv..
..v.>>vv..
..>>v>>vv.
..>>>>>vv.
v......>vv
v>v....>>v
vvv.....>>
>vv......>
.>v.vv.v..
"
        );
    }
}
