use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/day06.txt");

fn main() {
    let fish = parse_input(INPUT);

    {
        // Part 1
        let mut fish = fish.clone();
        (0..80).for_each(|_| step_day(&mut fish));
        println!("Answer 1: {}", fish.len());
    }

    // Part 2
    let mut shoal = Shoal::from(fish);
    (0..256).for_each(|_| shoal.step_day());
    println!("Answer 2: {}", shoal.size());
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Fish {
    timer: u8,
}

impl Fish {
    fn step_day(&mut self) -> Option<Fish> {
        match self.timer {
            0 => {
                self.timer = 6;
                Some(Fish { timer: 8 })
            }
            _ => {
                self.timer -= 1;
                None
            }
        }
    }
}

impl PartialEq<u8> for Fish {
    fn eq(&self, other: &u8) -> bool {
        self.timer == *other
    }
}

impl From<u8> for Fish {
    fn from(timer: u8) -> Self {
        Fish { timer }
    }
}

fn step_day(fish: &mut Vec<Fish>) {
    let new_fish: Vec<Fish> = fish.iter_mut().filter_map(|f| f.step_day()).collect();
    fish.extend(new_fish);
}

struct Shoal {
    fish: HashMap<Fish, u64>,
}

impl From<Vec<Fish>> for Shoal {
    fn from(fish: Vec<Fish>) -> Self {
        let mut map = HashMap::<Fish, u64>::new();
        for f in fish {
            if let Some(count) = map.get_mut(&f) {
                *count += 1;
            } else {
                map.insert(f, 1);
            }
        }
        Self { fish: map }
    }
}

impl Shoal {
    fn size(&self) -> u64 {
        self.fish.iter().map(|(_, count)| count).sum()
    }
    fn step_day(&mut self) {
        let new_fish: Vec<_> = self
            .fish
            .iter()
            .map(|(fish, count)| {
                let mut fish = fish.clone();
                let newborn = fish.step_day();
                (fish, newborn, *count)
            })
            .collect();

        let mut new_fish_map = HashMap::new();
        for (f, newborn, c) in new_fish {
            if let Some(count) = new_fish_map.get_mut(&f) {
                *count += c;
            } else {
                new_fish_map.insert(f, c);
            }
            if let Some(newborn) = newborn {
                new_fish_map.insert(newborn, c);
            }
        }

        self.fish = new_fish_map;
    }
}

fn parse_input(input: &str) -> Vec<Fish> {
    input
        .trim()
        .split(',')
        .map(|e| Fish {
            timer: e.parse().unwrap(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EXAMPLE: &str = "3,4,3,1,2
";

    #[test]
    fn example() {
        let fish = parse_input(INPUT_EXAMPLE);
        {
            // Part 1
            let mut fish = fish.clone();
            assert_eq!(fish, [3, 4, 3, 1, 2]);

            step_day(&mut fish);
            assert_eq!(fish, [2, 3, 2, 0, 1]);

            step_day(&mut fish);
            assert_eq!(fish, [1, 2, 1, 6, 0, 8]);

            step_day(&mut fish);
            assert_eq!(fish, [0, 1, 0, 5, 6, 7, 8]);

            (0..15).for_each(|_| step_day(&mut fish));
            assert_eq!(
                fish,
                [6, 0, 6, 4, 5, 6, 0, 1, 1, 2, 6, 0, 1, 1, 1, 2, 2, 3, 3, 4, 6, 7, 8, 8, 8, 8]
            );

            (0..62).for_each(|_| step_day(&mut fish));
            assert_eq!(fish.len(), 5934);
        }

        // Part 2
        let mut shoal = Shoal::from(fish);
        assert_eq!(*shoal.fish.get(&3.into()).unwrap(), 2);
        assert_eq!(shoal.size(), 5);

        shoal.step_day();
        assert_eq!(shoal.size(), 5);
        shoal.step_day();
        assert_eq!(shoal.size(), 6);
        shoal.step_day();
        assert_eq!(shoal.size(), 7);

        (0..15).for_each(|_| shoal.step_day());
        assert_eq!(shoal.size(), 26);

        (0..62).for_each(|_| shoal.step_day());
        assert_eq!(shoal.size(), 5934);

        (0..176).for_each(|_| shoal.step_day());
        assert_eq!(shoal.size(), 26984457539);
    }
}
