#![feature(int_abs_diff)]

use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/day05.txt");

fn main() {
    let lines = parse_input(INPUT);

    // Part 1
    let horizonal_or_vertical_lines = horizontal_or_vertical_lines(&lines);
    let mut intersection_map = IntersectionMap::default();

    horizonal_or_vertical_lines
        .iter()
        .for_each(|line| intersection_map.add_line(line));

    println!("Answer 1: {}", intersection_map.points_with_overlap().len());

    // Part 2
    let mut intersection_map = IntersectionMap::default();
    lines
        .iter()
        .for_each(|line| intersection_map.add_line(line));
    println!("Answer 2: {}", intersection_map.points_with_overlap().len());
}

#[derive(Debug, PartialEq, Clone)]
struct Line {
    x1: u16,
    y1: u16,
    x2: u16,
    y2: u16,
}

impl Line {
    fn is_horizontal_or_vertical(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }

    fn points(&self) -> Vec<(u16, u16)> {
        if self.x1 == self.x2 {
            let x = self.x1;
            ((u16::min(self.y1, self.y2))..=(u16::max(self.y1, self.y2)))
                .map(|y| (x, y))
                .collect()
        } else if self.y1 == self.y2 {
            let y = self.y1;
            ((u16::min(self.x1, self.x2))..=(u16::max(self.x1, self.x2)))
                .map(|x| (x, y))
                .collect()
        } else {
            // Only ever horizontal, vertical, or diagonals with 45 degrees
            assert!(self.x1.abs_diff(self.x2) == self.y1.abs_diff(self.y2));

            let x_inc = if self.x1 < self.x2 { 1 } else { -1 };
            let y_inc = if self.y1 < self.y2 { 1 } else { -1 };

            (0..=(self.x1.abs_diff(self.x2) as i16))
                .map(|i| {
                    (
                        (self.x1 as i16 + i * x_inc) as u16,
                        (self.y1 as i16 + i * y_inc) as u16,
                    )
                })
                .collect()
        }
    }
}

fn horizontal_or_vertical_lines(lines: &[Line]) -> Vec<&Line> {
    lines
        .iter()
        .filter(|l| l.is_horizontal_or_vertical())
        .collect()
}

impl std::str::FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<u16> = s
            .split(" -> ")
            .flat_map(|p| p.split(',').map(|e| e.parse().unwrap()))
            .collect();
        let (x1, y1, x2, y2) = (v[0], v[1], v[2], v[3]);
        Ok(Line { x1, y1, x2, y2 })
    }
}

#[derive(Default)]
struct IntersectionMap {
    map: HashMap<(u16, u16), u16>,
}

impl IntersectionMap {
    fn add_point(&mut self, (x, y): (u16, u16)) {
        if let Some(count) = self.map.get_mut(&(x, y)) {
            *count += 1;
        } else {
            self.map.insert((x, y), 1);
        }
    }

    fn add_line(&mut self, line: &Line) {
        line.points()
            .iter()
            .for_each(|point| self.add_point(*point));
    }

    fn points_with_overlap(&self) -> Vec<(u16, u16)> {
        self.map
            .iter()
            .filter_map(|(point, count)| if *count > 1 { Some(point) } else { None })
            .cloned()
            .collect()
    }
}

fn parse_input(input: &str) -> Vec<Line> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EXAMPLE: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

    #[test]
    fn example() {
        // Input
        let lines = parse_input(INPUT_EXAMPLE);
        assert_eq!(
            lines,
            [
                Line {
                    x1: 0,
                    y1: 9,
                    x2: 5,
                    y2: 9
                },
                Line {
                    x1: 8,
                    y1: 0,
                    x2: 0,
                    y2: 8
                },
                Line {
                    x1: 9,
                    y1: 4,
                    x2: 3,
                    y2: 4
                },
                Line {
                    x1: 2,
                    y1: 2,
                    x2: 2,
                    y2: 1
                },
                Line {
                    x1: 7,
                    y1: 0,
                    x2: 7,
                    y2: 4
                },
                Line {
                    x1: 6,
                    y1: 4,
                    x2: 2,
                    y2: 0
                },
                Line {
                    x1: 0,
                    y1: 9,
                    x2: 2,
                    y2: 9
                },
                Line {
                    x1: 3,
                    y1: 4,
                    x2: 1,
                    y2: 4
                },
                Line {
                    x1: 0,
                    y1: 0,
                    x2: 8,
                    y2: 8
                },
                Line {
                    x1: 5,
                    y1: 5,
                    x2: 8,
                    y2: 2
                },
            ]
        );

        // Part 1
        let horizonal_or_vertical_lines = horizontal_or_vertical_lines(&lines);
        assert_eq!(
            horizonal_or_vertical_lines,
            [
                &Line {
                    x1: 0,
                    y1: 9,
                    x2: 5,
                    y2: 9
                },
                &Line {
                    x1: 9,
                    y1: 4,
                    x2: 3,
                    y2: 4
                },
                &Line {
                    x1: 2,
                    y1: 2,
                    x2: 2,
                    y2: 1
                },
                &Line {
                    x1: 7,
                    y1: 0,
                    x2: 7,
                    y2: 4
                },
                &Line {
                    x1: 0,
                    y1: 9,
                    x2: 2,
                    y2: 9
                },
                &Line {
                    x1: 3,
                    y1: 4,
                    x2: 1,
                    y2: 4
                },
            ]
        );

        let example_line = Line {
            x1: 9,
            y1: 4,
            x2: 3,
            y2: 4,
        };
        assert_eq!(
            example_line.points(),
            [(3, 4), (4, 4), (5, 4), (6, 4), (7, 4), (8, 4), (9, 4)]
        );

        let mut intersection_map = IntersectionMap::default();
        horizonal_or_vertical_lines
            .iter()
            .for_each(|line| intersection_map.add_line(line));

        assert_eq!(intersection_map.points_with_overlap().len(), 5);

        // Part 2
        let diagonal_line = Line {
            x1: 8,
            y1: 0,
            x2: 0,
            y2: 8,
        };
        assert_eq!(
            diagonal_line.points(),
            [
                (8, 0),
                (7, 1),
                (6, 2),
                (5, 3),
                (4, 4),
                (3, 5),
                (2, 6),
                (1, 7),
                (0, 8)
            ]
        );

        let mut intersection_map = IntersectionMap::default();
        lines
            .iter()
            .for_each(|line| intersection_map.add_line(line));
        assert_eq!(intersection_map.points_with_overlap().len(), 12);
    }
}
