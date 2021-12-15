use std::collections::BinaryHeap;

const INPUT: &str = include_str!("../../inputs/day15.txt");

fn main() {
    let cavern = parse_input(INPUT);
    println!("Answer 1: {}", lowest_total_risk(&cavern));
    let cavern_expanded = expand_cavern(cavern);
    println!("Answer 2: {}", lowest_total_risk(&cavern_expanded));
}

fn parse_input(s: &str) -> Vec<Vec<u8>> {
    s.lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

#[derive(Eq, Default, Clone, Debug)]
struct Path {
    risk: i32,
    current: (usize, usize),
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.risk.cmp(&other.risk)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.risk == other.risk
    }
}

fn lowest_total_risk(cavern: &[Vec<u8>]) -> usize {
    let height = cavern.len();
    let width = cavern[0].len();
    let mut priority_queue = BinaryHeap::from([Path::default()]);
    let mut visited = vec![vec![false; width]; height];

    while let Some(path) = priority_queue.pop() {
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let (x, y) = (path.current.0 as i32 + dx, path.current.1 as i32 + dy);
            if x < 0 || y < 0 || x as usize > width - 1 || y as usize > height - 1 {
                continue;
            }
            let (x, y) = (x as usize, y as usize);
            if (x, y) == (width - 1, height - 1) {
                return ((cavern[y][x] as i32) - path.risk) as usize;
            }

            if !visited[y][x] {
                let new_path = Path {
                    current: (x, y),
                    risk: path.risk - cavern[y][x] as i32,
                };
                priority_queue.push(new_path);
                visited[y][x] = true;
            }
        }
    }

    unreachable!()
}

fn expand_cavern(cavern: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let height = cavern.len();
    let width = cavern[0].len();
    let mut cavern_expanded = vec![vec![0; width * 5]; height * 5];

    for x in 0..width {
        for y in 0..height {
            for mx in 0..5 {
                for my in 0..5 {
                    let mut new_value = (cavern[y][x] + mx + my) % 9;
                    if new_value == 0 {
                        new_value = 9;
                    }
                    cavern_expanded[y + height * my as usize][x + width * mx as usize] = new_value;
                }
            }
        }
    }

    cavern_expanded
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EXAMPLE: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";

    const INPUT_EXAMPLE_EXPANDED: &str = "11637517422274862853338597396444961841755517295286
13813736722492484783351359589446246169155735727126
21365113283247622439435873354154698446526571955763
36949315694715142671582625378269373648937148475914
74634171118574528222968563933317967414442817852555
13191281372421239248353234135946434524615754563572
13599124212461123532357223464346833457545794456865
31254216394236532741534764385264587549637569865174
12931385212314249632342535174345364628545647573965
23119445813422155692453326671356443778246755488935
22748628533385973964449618417555172952866628316397
24924847833513595894462461691557357271266846838237
32476224394358733541546984465265719557637682166874
47151426715826253782693736489371484759148259586125
85745282229685639333179674144428178525553928963666
24212392483532341359464345246157545635726865674683
24611235323572234643468334575457944568656815567976
42365327415347643852645875496375698651748671976285
23142496323425351743453646285456475739656758684176
34221556924533266713564437782467554889357866599146
33859739644496184175551729528666283163977739427418
35135958944624616915573572712668468382377957949348
43587335415469844652657195576376821668748793277985
58262537826937364893714847591482595861259361697236
96856393331796741444281785255539289636664139174777
35323413594643452461575456357268656746837976785794
35722346434683345754579445686568155679767926678187
53476438526458754963756986517486719762859782187396
34253517434536462854564757396567586841767869795287
45332667135644377824675548893578665991468977611257
44961841755517295286662831639777394274188841538529
46246169155735727126684683823779579493488168151459
54698446526571955763768216687487932779859814388196
69373648937148475914825958612593616972361472718347
17967414442817852555392896366641391747775241285888
46434524615754563572686567468379767857948187896815
46833457545794456865681556797679266781878137789298
64587549637569865174867197628597821873961893298417
45364628545647573965675868417678697952878971816398
56443778246755488935786659914689776112579188722368
55172952866628316397773942741888415385299952649631
57357271266846838237795794934881681514599279262561
65719557637682166874879327798598143881961925499217
71484759148259586125936169723614727183472583829458
28178525553928963666413917477752412858886352396999
57545635726865674683797678579481878968159298917926
57944568656815567976792667818781377892989248891319
75698651748671976285978218739618932984172914319528
56475739656758684176786979528789718163989182927419
67554889357866599146897761125791887223681299833479
";

    #[test]
    fn part1() {
        let cavern = parse_input(INPUT_EXAMPLE);
        assert_eq!(lowest_total_risk(&cavern), 40);
    }

    #[test]
    fn part2() {
        let cavern = parse_input(INPUT_EXAMPLE);
        let cavern_expanded = parse_input(INPUT_EXAMPLE_EXPANDED);
        assert_eq!(expand_cavern(cavern), cavern_expanded);
        assert_eq!(lowest_total_risk(&cavern_expanded), 315);
    }
}
