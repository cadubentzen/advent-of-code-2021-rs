use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../inputs/day09.txt");

fn main() {
    let heights = parse_input(INPUT);

    println!("Answer 1: {}", sum_risk_low_points(&heights));
    println!("Answer 2: {}", product_three_largest_basins(&heights));
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| Vec::from_iter(line.chars().map(|c| c.to_digit(10).unwrap() as u8)))
        .collect()
}

fn sum_risk_low_points(h: &[Vec<u8>]) -> usize {
    let mut low_points = vec![];

    let num_rows = h.len();
    let num_cols = h[0].len();

    for i in 0..num_rows {
        for j in 0..num_cols {
            let mut cond = true;
            'checking: for ii in [-1, 0, 1] {
                if (i == 0 && ii == -1) || (i == num_rows - 1 && ii == 1) {
                    continue;
                }
                for jj in [-1, 0, 1] {
                    if (j == 0 && jj == -1)
                        || (j == num_cols - 1 && jj == 1)
                        || (ii != 0 && jj != 0)
                        || (ii == 0 && jj == 0)
                    {
                        continue;
                    }
                    cond =
                        cond && (h[i][j] < h[(i as i32 + ii) as usize][(j as i32 + jj) as usize]);
                    if !cond {
                        break 'checking;
                    }
                }
            }
            if cond {
                low_points.push(h[i][j]);
            }
        }
    }

    low_points.into_iter().map(|l| (l + 1) as usize).sum()
}

fn get_basin(
    points: &mut HashMap<(usize, usize), usize>,
    links: &mut HashMap<usize, Option<usize>>,
    i: usize,
    j: usize,
) -> usize {
    let mut basin = *points.get(&(i, j)).unwrap();
    while let Some(b) = links.get(&basin).unwrap() {
        basin = *b;
    }
    basin
}

fn product_three_largest_basins(h: &[Vec<u8>]) -> usize {
    let num_rows = h.len();
    let num_cols = h[0].len();

    let mut tag = 0;
    let mut sizes = HashMap::<usize, usize>::new();
    let mut points = HashMap::<(usize, usize), usize>::new();
    let mut links = HashMap::<usize, Option<usize>>::new();

    for i in 0..num_rows {
        for j in 0..num_cols {
            if h[i][j] == 9 {
                continue;
            }
            let mut neighbor_tags = HashSet::new();
            for ii in [-1, 0, 1] {
                if (i == 0 && ii == -1) || (i == num_rows - 1 && ii == 1) {
                    continue;
                }
                for jj in [-1, 0, 1] {
                    if (j == 0 && jj == -1)
                        || (j == num_cols - 1 && jj == 1)
                        || (ii != 0 && jj != 0)
                        || (ii == 0 && jj == 0)
                    {
                        continue;
                    }
                    let (iii, jjj) = ((i as i32 + ii) as usize, (j as i32 + jj) as usize);
                    if h[iii][jjj] == 9 {
                        continue;
                    }
                    if points.contains_key(&(iii, jjj)) {
                        neighbor_tags.insert(get_basin(&mut points, &mut links, iii, jjj));
                    }
                }
            }
            if neighbor_tags.is_empty() {
                points.insert((i as usize, j as usize), tag);
                links.insert(tag, None);
                sizes.insert(tag, 1);
                tag += 1;
            } else {
                let largest_tag = *neighbor_tags.iter().max_by_key(|t| sizes.get(t)).unwrap();
                points.insert((i, j), largest_tag);
                *sizes.get_mut(&largest_tag).unwrap() += 1;
                if neighbor_tags.len() > 1 {
                    neighbor_tags.remove(&largest_tag);
                    let mut size_sums = 0;
                    for t in neighbor_tags {
                        links.insert(t, Some(largest_tag));
                        size_sums += sizes.remove(&t).unwrap();
                    }
                    *sizes.get_mut(&largest_tag).unwrap() += size_sums;
                }
            }
        }
    }

    let mut sizes = sizes.into_values().collect::<Vec<_>>();
    sizes.sort_unstable();

    sizes.into_iter().rev().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EXAMPLE: &str = "2199943210
3987894921
9856789892
8767896789
9899965678
";

    #[test]
    fn example() {
        let heights = parse_input(INPUT_EXAMPLE);
        assert_eq!(
            heights,
            [
                [2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
                [3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
                [9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
                [8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
                [9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
            ]
        );

        assert_eq!(sum_risk_low_points(&heights), 15);
        assert_eq!(product_three_largest_basins(&heights), 1134);
    }
}
