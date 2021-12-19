#![feature(drain_filter)]
#![feature(int_abs_diff)]

const INPUT: &str = include_str!("../../inputs/day19.txt");

fn main() {
    let scanners = parse_input(INPUT);
    let (scanner_positions, beacon_positions) = find_positions(scanners);
    println!("Answer 1: {}", beacon_positions.len());
    println!(
        "Answer 2: {}",
        largest_manhattan_distance(scanner_positions)
    );
}

#[derive(Debug)]
struct Scanner {
    beacon_relative_positions: HashSet<Point>,
    solved: bool,
}

struct SolvedScanner {
    position: Point,
    beacon_relative_positions: HashSet<Point>,
    beacon_absolute_positions: HashSet<Point>,
}

fn parse_input(s: &str) -> Vec<Scanner> {
    s.split("\n\n")
        .map(|scan_section| Scanner {
            solved: false,
            beacon_relative_positions: scan_section
                .lines()
                .skip(1)
                .map(|line| line.parse().unwrap())
                .collect(),
        })
        .collect()
}

fn find_positions(mut scanners: Vec<Scanner>) -> (HashSet<Point>, HashSet<Point>) {
    let mut beacons = HashSet::new();
    let mut scanner_positions = HashSet::new();
    let mut solved_scanners = VecDeque::<SolvedScanner>::new();

    let first = SolvedScanner {
        position: Point::new(0, 0, 0),
        beacon_relative_positions: scanners[0].beacon_relative_positions.clone(),
        beacon_absolute_positions: scanners[0].beacon_relative_positions.clone(),
    };
    solved_scanners.push_back(first);
    scanners[0].solved = true;

    while !scanners.is_empty() {
        scanners.drain_filter(|s| s.solved).for_each(drop);
        let solved_scanner = solved_scanners.pop_front().unwrap();
        beacons.extend(solved_scanner.beacon_absolute_positions.iter().cloned());
        scanner_positions.insert(solved_scanner.position.clone());

        let solved_beacons = &solved_scanner.beacon_relative_positions;
        for scanner in scanners.iter_mut() {
            if scanner.solved {
                continue;
            }

            'orientation: for orientation in &ORIENTATIONS {
                let orientated_beacons = scanner
                    .beacon_relative_positions
                    .iter()
                    .map(|b| b.rotate(orientation))
                    .collect::<Vec<_>>();

                for beacon in &orientated_beacons {
                    for solved_beacon in solved_beacons {
                        let dist = beacon.dist(solved_beacon);
                        let same_distances = orientated_beacons
                            .iter()
                            .filter(|b| solved_beacons.contains(&b.add(&dist)))
                            .count();
                        if same_distances >= 12 {
                            solved_scanners.push_back(SolvedScanner {
                                position: solved_scanner.position.add(&dist),
                                beacon_relative_positions: HashSet::from_iter(
                                    orientated_beacons.clone(),
                                ),
                                beacon_absolute_positions: orientated_beacons
                                    .iter()
                                    .cloned()
                                    .map(|b| {
                                        let pos = b.add(&dist);
                                        pos.add(&solved_scanner.position)
                                    })
                                    .collect(),
                            });
                            scanner.solved = true;
                            break 'orientation;
                        }
                    }
                }
            }
        }
    }

    for solved_scanner in solved_scanners {
        beacons.extend(solved_scanner.beacon_absolute_positions);
        scanner_positions.insert(solved_scanner.position);
    }

    (scanner_positions, beacons)
}

fn largest_manhattan_distance(positions: HashSet<Point>) -> u16 {
    let mut max_distance = 0;
    for c in positions.iter().combinations(2) {
        max_distance = max_distance
            .max(c[0].x.abs_diff(c[1].x) + c[0].y.abs_diff(c[1].y) + c[0].z.abs_diff(c[1].z))
    }
    max_distance
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i16,
    y: i16,
    z: i16,
}

impl Point {
    fn new(x: i16, y: i16, z: i16) -> Self {
        Self { x, y, z }
    }

    fn rotate(&self, rotation: &Orientation) -> Self {
        let x = rotation.x.get_value(self);
        let y = rotation.y.get_value(self);
        let z = rotation.z.get_value(self);
        Self { x, y, z }
    }

    fn dist(&self, other: &Self) -> Self {
        let x = other.x - self.x;
        let y = other.y - self.y;
        let z = other.z - self.z;
        Self { x, y, z }
    }

    fn add(&self, other: &Self) -> Self {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;
        Self { x, y, z }
    }
}

impl std::str::FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [x, y, z]: [&str; 3] = s.split(',').collect::<Vec<_>>().try_into().unwrap();
        let (x, y, z) = (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap());
        Ok(Self { x, y, z })
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone)]
enum Axis {
    X(bool),
    Y(bool),
    Z(bool),
}

use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

use itertools::Itertools;
use Axis::{X, Y, Z};

impl Axis {
    fn get_value(&self, point: &Point) -> i16 {
        match self {
            X(true) => point.x,
            X(false) => -point.x,
            Y(true) => point.y,
            Y(false) => -point.y,
            Z(true) => point.z,
            Z(false) => -point.z,
        }
    }
}

#[derive(Debug, Clone)]
struct Orientation {
    x: Axis,
    y: Axis,
    z: Axis,
}

impl Orientation {
    const fn new(x: Axis, y: Axis, z: Axis) -> Self {
        assert!(!matches!(
            (&x, &y, &z),
            (X(_), X(_), X(_))
                | (X(_), X(_), Y(_))
                | (X(_), X(_), Z(_))
                | (X(_), Y(_), X(_))
                | (X(_), Y(_), Y(_))
                | (X(_), Z(_), X(_))
                | (X(_), Z(_), Z(_))
                | (Y(_), X(_), X(_))
                | (Y(_), X(_), Y(_))
                | (Y(_), Y(_), X(_))
                | (Y(_), Y(_), Y(_))
                | (Y(_), Y(_), Z(_))
                | (Y(_), Z(_), Y(_))
                | (Y(_), Z(_), Z(_))
                | (Z(_), X(_), X(_))
                | (Z(_), X(_), Z(_))
                | (Z(_), Y(_), Y(_))
                | (Z(_), Z(_), X(_))
                | (Z(_), Z(_), Y(_))
                | (Z(_), Z(_), Z(_))
        ));

        Self { x, y, z }
    }
}

const ORIENTATIONS: [Orientation; 24] = [
    // X
    Orientation::new(X(true), Y(true), Z(true)),
    Orientation::new(X(true), Z(true), Y(false)),
    Orientation::new(X(true), Y(false), Z(false)),
    Orientation::new(X(true), Z(false), Y(true)),
    // -X
    Orientation::new(X(false), Y(false), Z(true)),
    Orientation::new(X(false), Z(true), Y(true)),
    Orientation::new(X(false), Y(true), Z(false)),
    Orientation::new(X(false), Z(false), Y(false)),
    // Y
    Orientation::new(Y(true), X(false), Z(true)),
    Orientation::new(Y(true), Z(true), X(true)),
    Orientation::new(Y(true), X(true), Z(false)),
    Orientation::new(Y(true), Z(false), X(false)),
    // -Y
    Orientation::new(Y(false), X(true), Z(true)),
    Orientation::new(Y(false), Z(true), X(false)),
    Orientation::new(Y(false), X(false), Z(false)),
    Orientation::new(Y(false), Z(false), X(true)),
    // Z
    Orientation::new(Z(true), Y(false), X(true)),
    Orientation::new(Z(true), X(true), Y(true)),
    Orientation::new(Z(true), Y(true), X(false)),
    Orientation::new(Z(true), X(false), Y(false)),
    // -Z
    Orientation::new(Z(false), Y(true), X(true)),
    Orientation::new(Z(false), X(true), Y(false)),
    Orientation::new(Z(false), Y(false), X(false)),
    Orientation::new(Z(false), X(false), Y(true)),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_rotation() {
        let point = Point::new(1, 2, 3);
        assert_eq!(
            point.rotate(&Orientation::new(X(true), Y(true), Z(true))),
            Point::new(1, 2, 3)
        );
        assert_eq!(
            point.rotate(&Orientation::new(X(false), Y(true), Z(true))),
            Point::new(-1, 2, 3)
        );
        assert_eq!(
            point.rotate(&Orientation::new(Y(false), X(true), Z(false))),
            Point::new(-2, 1, -3)
        );
    }

    const INPUT_EXAMPLE: &str = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14
";

    #[test]
    fn example() {
        let scanners = parse_input(INPUT_EXAMPLE);
        let (scanner_positions, beacon_positions) = find_positions(scanners);
        assert_eq!(beacon_positions.len(), 79);
        assert_eq!(largest_manhattan_distance(scanner_positions), 3621);
    }
}
