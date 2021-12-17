use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../inputs/day17.txt");

fn main() {
    let target: Target = INPUT.parse().unwrap();
    println!("Answer 1: {}", target.max_y());
    println!("Answer 2: {}", target.num_initial_velocities());
}

#[derive(Debug)]
struct Target {
    x_min: u32,
    x_max: u32,
    y_min: i32,
    y_max: i32,
}

impl Target {
    fn new(x_min: u32, x_max: u32, y_min: i32, y_max: i32) -> Self {
        // Working only with negative y targets here for some assumptions
        // in maths.
        assert!(y_min < 0);
        assert!(y_max < 0);
        Self {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    fn max_y(&self) -> i32 {
        let vy0 = self.max_vy0();
        vy0 * (vy0 + 1) / 2
    }

    fn max_vy0(&self) -> i32 {
        -self.y_min - 1
    }

    fn min_vy0(&self) -> i32 {
        self.y_min
    }

    fn max_vx0(&self) -> u32 {
        self.x_max
    }

    fn min_vx0(&self) -> u32 {
        1
    }

    fn num_initial_velocities(&self) -> usize {
        let mut steps_y = HashMap::<usize, HashSet<i32>>::new();
        for vy0 in self.min_vy0()..=self.max_vy0() {
            let mut vy = vy0;
            let mut y = 0;
            let mut step = 0;
            while y > self.y_max {
                step += 1;
                y += vy;
                vy -= 1;
            }

            while y <= self.y_max && y >= self.y_min {
                steps_y.entry(step).or_default().insert(vy0);
                step += 1;
                y += vy;
                vy -= 1;
            }
        }
        let max_steps = *steps_y.keys().max().unwrap();

        let mut steps_x = HashMap::<usize, HashSet<u32>>::new();
        for vx0 in self.min_vx0()..=self.max_vx0() {
            let mut vx = vx0;
            let mut x = 0;
            let mut step = 0;
            while x < self.x_min && vx > 0 {
                step += 1;
                x += vx;
                vx = vx.saturating_sub(1);
            }

            while x >= self.x_min && x <= self.x_max && step <= max_steps {
                steps_x.entry(step).or_default().insert(vx0);
                step += 1;
                if vx > 0 {
                    x += vx;
                    vx = vx.saturating_sub(1);
                }
            }
        }

        let mut combinations = HashSet::new();
        for (steps, vy0) in steps_y {
            if steps_x.contains_key(&steps) {
                combinations.extend(
                    vy0.into_iter()
                        .cartesian_product(steps_x.get(&steps).unwrap().iter()),
                );
            }
        }

        combinations.len()
    }
}

impl std::str::FromStr for Target {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, ranges) = s.trim().split_once(": ").unwrap();
        let (x_range, y_range) = ranges.split_once(", ").unwrap();

        let (_, x_range_values) = x_range.split_once('=').unwrap();
        let (x_min, x_max) = x_range_values.split_once("..").unwrap();
        let (x_min, x_max) = (x_min.parse().unwrap(), x_max.parse().unwrap());

        let (_, y_range_values) = y_range.split_once('=').unwrap();
        let (y_min, y_max) = y_range_values.split_once("..").unwrap();
        let (y_min, y_max) = (y_min.parse().unwrap(), y_max.parse().unwrap());

        Ok(Target::new(x_min, x_max, y_min, y_max))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EXAMPLE: &str = "target area: x=20..30, y=-10..-5\n";

    #[test]
    fn example() {
        let target: Target = dbg!(INPUT_EXAMPLE.parse().unwrap());
        assert_eq!(target.max_y(), 45);
        assert_eq!(target.num_initial_velocities(), 112);
    }
}
