const INPUT: &str = include_str!("../../inputs/day20.txt");

fn main() {
    let (algorithm, input_image) = parse_input(INPUT);
    let enhanced = enhance(input_image.clone(), &algorithm);
    let enhanced2 = enhance(enhanced, &algorithm);
    println!("Answer 1: {}", enhanced2.num_pixels_lit());

    let enhanced50 = enhance_multiple(input_image, &algorithm, 50);
    println!("Answer 2: {}", enhanced50.num_pixels_lit());
}

#[derive(Debug, Clone)]
struct Image {
    pixels: Vec<Vec<bool>>,
    boundary: usize,
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = Ok(());
        for line in &self.pixels {
            for pixel in line {
                let _ = write!(f, "{}", if *pixel { '#' } else { '.' });
            }
            result = writeln!(f);
        }
        result
    }
}

impl Image {
    fn num_pixels_lit(&self) -> usize {
        self.pixels
            .iter()
            .map(|line| {
                line.iter()
                    .fold(0, |acc, pixel| if *pixel { acc + 1 } else { acc })
            })
            .sum()
    }
}

fn parse_input(s: &str) -> (Vec<bool>, Image) {
    let (algorithm, input_image) = s.trim().split_once("\n\n").unwrap();

    let char_to_bool = |c| match c {
        '.' => false,
        '#' => true,
        _ => panic!("Unknown char {}", c),
    };

    let algorithm = algorithm.chars().map(char_to_bool).collect();
    let input_image = input_image
        .lines()
        .map(|line| line.chars().map(char_to_bool).collect())
        .collect();
    (
        algorithm,
        Image {
            pixels: input_image,
            boundary: 0,
        },
    )
}

fn enhance(input: Image, algorithm: &[bool]) -> Image {
    let input_width = input.pixels[0].len();
    let input_height = input.pixels.len();
    let mut output_image = vec![vec![false; input_width + 2]; input_height + 2];

    for (i, line) in output_image.iter_mut().enumerate() {
        for (j, pixel) in line.iter_mut().enumerate() {
            let ii = i as isize - 1;
            let jj = j as isize - 1;
            let mut binary_sequence = 0usize;
            for (di, dj) in [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 0),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ] {
                let iii = ii + di;
                let jjj = jj + dj;
                let bit_val = if iii < 0
                    || jjj < 0
                    || iii as usize >= input_height
                    || jjj as usize >= input_width
                {
                    input.boundary
                } else if input.pixels[iii as usize][jjj as usize] {
                    1
                } else {
                    0
                };
                binary_sequence <<= 1;
                binary_sequence |= bit_val;
            }
            if algorithm[binary_sequence] {
                *pixel = true;
            }
        }
    }

    let boundary = (0..9).fold(0, |acc, _| (acc << 1) | input.boundary);
    let boundary = if algorithm[boundary] { 1 } else { 0 };

    Image {
        pixels: output_image,
        boundary,
    }
}

fn enhance_multiple(input: Image, algorithm: &[bool], n: usize) -> Image {
    (0..n).fold(input, |acc, _| enhance(acc, algorithm))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EXAMPLE: &str = "
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
";

    #[test]
    fn example() {
        let (algorithm, input_image) = parse_input(INPUT_EXAMPLE);
        let enhanced = enhance(input_image.clone(), &algorithm);
        assert_eq!(
            enhanced.to_string(),
            ".##.##.
#..#.#.
##.#..#
####..#
.#..##.
..##..#
...#.#.
"
        );
        let enhanced = enhance(enhanced, &algorithm);
        assert_eq!(
            enhanced.to_string(),
            ".......#.
.#..#.#..
#.#...###
#...##.#.
#.....#.#
.#.#####.
..#.#####
...##.##.
....###..
"
        );
        assert_eq!(enhanced.num_pixels_lit(), 35);

        let enhanced2 = enhance_multiple(input_image.clone(), &algorithm, 2);
        assert_eq!(
            enhanced2.to_string(),
            ".......#.
.#..#.#..
#.#...###
#...##.#.
#.....#.#
.#.#####.
..#.#####
...##.##.
....###..
"
        );
        assert_eq!(enhanced2.num_pixels_lit(), 35);

        let enhanced50 = enhance_multiple(input_image, &algorithm, 50);
        assert_eq!(enhanced50.num_pixels_lit(), 3351);
    }
}
