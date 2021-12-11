const INPUT: &str = include_str!("../../inputs/day11.txt");

fn main() {
    let mut energy_levels: EnergyLevels = INPUT.parse().unwrap();
    println!("Answer 1: {}", energy_levels.step(100));

    let mut energy_levels: EnergyLevels = INPUT.parse().unwrap();
    println!("Answer 2: {}", energy_levels.steps_to_sync());
}

#[derive(Debug, PartialEq)]
struct EnergyLevels {
    levels: [[u8; 10]; 10],
}

impl std::str::FromStr for EnergyLevels {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            levels: s
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap() as u8)
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        })
    }
}

impl EnergyLevels {
    fn flash(&mut self, i: usize, j: usize) -> usize {
        let mut num_flashes = 0;
        self.levels[i][j] = 0;
        num_flashes += 1;
        for ii in [-1, 0, 1] {
            if (i == 0 && ii == -1) || (i == 9 && ii == 1) {
                continue;
            }
            for jj in [-1, 0, 1] {
                if (ii == 0 && jj == 0) || (j == 0 && jj == -1) || (j == 9 && jj == 1) {
                    continue;
                }
                let (iii, jjj) = ((i as i32 + ii) as usize, (j as i32 + jj) as usize);
                if self.levels[iii][jjj] == 0 {
                    // already flashed
                    continue;
                }
                self.levels[iii][jjj] += 1;
                if self.levels[iii][jjj] > 9 {
                    num_flashes += self.flash(iii, jjj);
                }
            }
        }
        num_flashes
    }

    fn step(&mut self, num_steps: u8) -> usize {
        let mut num_flashes = 0;
        for _ in 0..num_steps {
            // First pass increases +1 in everyone
            for i in 0..10 {
                for j in 0..10 {
                    self.levels[i][j] += 1;
                }
            }
            // Second pass checks for the flashes
            for i in 0..10 {
                for j in 0..10 {
                    if self.levels[i][j] > 9 {
                        num_flashes += self.flash(i, j);
                    }
                }
            }
        }
        num_flashes
    }

    fn steps_to_sync(&mut self) -> usize {
        for i in std::iter::successors(Some(1), |n| Some(n + 1)) {
            if self.step(1) == 100 {
                return i;
            }
        }
        usize::MAX
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EXAMPLE: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";

    #[test]
    fn example() {
        let mut energy_levels: EnergyLevels = INPUT_EXAMPLE.parse().unwrap();

        assert_eq!(
            energy_levels.levels,
            [
                [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
                [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
                [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
                [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
                [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
                [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
                [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
                [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
                [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
                [5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
            ]
        );

        let num_flashes = energy_levels.step(1);
        assert_eq!(num_flashes, 0);
        assert_eq!(
            energy_levels,
            "6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637"
                .parse()
                .unwrap()
        );

        let num_flashes = energy_levels.step(1);
        assert_eq!(
            energy_levels,
            "8807476555
5089087054
8597889608
8485769600
8700908800
6600088989
6800005943
0000007456
9000000876
8700006848"
                .parse()
                .unwrap()
        );
        assert_eq!(num_flashes, 35);

        let num_flashes = energy_levels.step(1);
        assert_eq!(
            energy_levels,
            "0050900866
8500800575
9900000039
9700000041
9935080063
7712300000
7911250009
2211130000
0421125000
0021119000"
                .parse()
                .unwrap()
        );
        assert_eq!(num_flashes, 45);

        let mut energy_levels: EnergyLevels = INPUT_EXAMPLE.parse().unwrap();
        assert_eq!(energy_levels.step(10), 204);

        let mut energy_levels: EnergyLevels = INPUT_EXAMPLE.parse().unwrap();
        assert_eq!(energy_levels.step(100), 1656);

        // Part 2
        let mut energy_levels: EnergyLevels = INPUT_EXAMPLE.parse().unwrap();
        assert_eq!(energy_levels.steps_to_sync(), 195);
    }
}
