#![allow(incomplete_features)]
#![feature(int_abs_diff)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]

use std::{collections::HashMap, fmt::Display, hash::Hash};

const INPUT: &str = include_str!("../../inputs/day23.txt");
const INPUT_EXAMPLE3: &str = "##############
#............#
###B#C#B#D####
  #D#C#B#A#
  #D#B#A#C#
  #D#C#B#A#
  #D#B#A#C#
  #A#D#C#A#
  #########
";

fn main() {
    let game_state: GameState<2> = INPUT.parse().unwrap();
    println!("Answer 1: {}", solve_least_amount_of_energy(&game_state));

    let game_state = unfold(&game_state);
    println!("Answer 2: {}", solve_least_amount_of_energy(&game_state));

    let game_state: GameState<6> = INPUT_EXAMPLE3.parse().unwrap();
    println!("Answer 3: {}", solve_least_amount_of_energy(&game_state));
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum Cell {
    Empty,
    Wall,
    Void,
    Occupied(Amphipod),
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Wall => write!(f, "#"),
            Cell::Void => write!(f, " "),
            Cell::Occupied(amphipod) => write!(f, "{}", amphipod),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    #[inline]
    fn from(c: char) -> Self {
        match c {
            'A' => Self::Amber,
            'B' => Self::Bronze,
            'C' => Self::Copper,
            'D' => Self::Desert,
            _ => unreachable!(),
        }
    }

    #[inline]
    fn column(&self) -> usize {
        match self {
            Self::Amber => 2,
            Self::Bronze => 4,
            Self::Copper => 6,
            Self::Desert => 8,
        }
    }

    #[inline]
    fn energy_per_step(&self) -> usize {
        match self {
            Self::Amber => 1,
            Self::Bronze => 10,
            Self::Copper => 100,
            Self::Desert => 1000,
        }
    }
}

impl Display for Amphipod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Amphipod::Amber => 'A',
                Amphipod::Bronze => 'B',
                Amphipod::Copper => 'C',
                Amphipod::Desert => 'D',
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Cells<const N: usize>([[Cell; 12]; (N + 1)])
where
    [(); (N + 1)]: Sized;

#[derive(Debug, Clone)]
struct GameState<const N: usize>
where
    [(); (N + 1)]: Sized,
{
    cells: Cells<N>,
    energy: usize,
}

impl<const N: usize> Ord for GameState<N>
where
    [(); (N + 1)]: Sized,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.energy.cmp(&self.energy)
    }
}

impl<const N: usize> PartialOrd for GameState<N>
where
    [(); (N + 1)]: Sized,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> PartialEq for GameState<N>
where
    [(); (N + 1)]: Sized,
{
    fn eq(&self, other: &Self) -> bool {
        self.energy == other.energy
    }
}

impl<const N: usize> Eq for GameState<N> where [(); (N + 1)]: Sized {}

const ROOM_COLUMNS: [usize; 4] = [2, 4, 6, 8];
const HALLWAY_COLUMNS_ALLOWED: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];

impl<const N: usize> GameState<N>
where
    [(); (N + 1)]: Sized,
{
    fn new(initial_amphipods: [[Amphipod; N]; 4]) -> Self {
        let mut cells = [[Cell::Void; 12]; N + 1];

        // hallway line
        for cell in &mut cells[0] {
            *cell = Cell::Empty;
        }

        // room lines
        for (i, room_line) in cells.iter_mut().skip(1).enumerate() {
            for (j, cell) in room_line.iter_mut().enumerate() {
                *cell = if ROOM_COLUMNS.contains(&j) {
                    Cell::Occupied(initial_amphipods[j / 2 - 1][i])
                } else {
                    Cell::Wall
                };
            }
        }

        // the two extra empty strips
        for line in cells.iter_mut().skip(3) {
            line[0] = Cell::Empty;
            line[10] = Cell::Empty;
        }

        Self {
            cells: Cells(cells),
            energy: 0,
        }
    }

    fn is_solved(&self) -> bool {
        for (kind, column) in [
            Amphipod::Amber,
            Amphipod::Bronze,
            Amphipod::Copper,
            Amphipod::Desert,
        ]
        .into_iter()
        .zip(ROOM_COLUMNS)
        {
            for i in 1..=N {
                match self.cells.0[i][column] {
                    Cell::Occupied(kind_in_room) => {
                        if kind_in_room != kind {
                            return false;
                        }
                    }
                    _ => return false,
                }
            }
        }

        true
    }

    fn hallway_path_is_clear(&self, from: usize, to: usize) -> bool {
        if from < to {
            for j in (from + 1)..=to {
                if !matches!(self.cells.0[0][j], Cell::Empty) {
                    return false;
                }
            }
        } else {
            for j in to..=(from - 1) {
                if !matches!(self.cells.0[0][j], Cell::Empty) {
                    return false;
                }
            }
        }
        true
    }

    fn room_path_is_clear(&self, column: usize, row: usize) -> bool {
        // in this case we leave for the hallway to check
        if row == 0 {
            return true;
        }
        for i in 1..=row {
            if self.cells.0[i][column] != Cell::Empty {
                return false;
            }
        }
        true
    }

    fn room_only_has_same_kind_or_empty(&self, column: usize, kind: Amphipod) -> bool {
        for i in 1..=N {
            match self.cells.0[i][column] {
                Cell::Empty => (),
                Cell::Wall => unreachable!(),
                Cell::Void => unreachable!(),
                Cell::Occupied(kind_in_room) => {
                    if kind_in_room != kind {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn moved_from_hallway_to_room(&self, column: usize) -> Option<Self> {
        if let Cell::Occupied(amphipod) = self.cells.0[0][column] {
            let target_column = amphipod.column();
            assert!(column != target_column);

            if !self.hallway_path_is_clear(column, target_column)
                || !self.room_only_has_same_kind_or_empty(target_column, amphipod)
            {
                return None;
            }

            for i in (1..=N).rev() {
                if self.room_path_is_clear(target_column, i) {
                    let mut next_state = self.clone();
                    next_state.cells.0[i][target_column] = Cell::Occupied(amphipod);
                    next_state.cells.0[0][column] = Cell::Empty;

                    next_state.energy +=
                        (i + column.abs_diff(target_column)) * amphipod.energy_per_step();
                    return Some(next_state);
                }
            }
        }

        None
    }

    fn moved_from_room_to_hallway(
        &self,
        (row, column): (usize, usize),
        target_column: usize,
    ) -> Option<Self> {
        if let Cell::Empty = self.cells.0[0][target_column] {
            if let Cell::Occupied(amphipod) = self.cells.0[row][column] {
                let final_column = amphipod.column();
                if column == final_column {
                    let mut already_positioned = true;
                    for i in row..=N {
                        if self.cells.0[i][column] != Cell::Occupied(amphipod) {
                            already_positioned = false;
                            break;
                        }
                    }
                    if already_positioned {
                        return None;
                    }
                }

                if !self.room_path_is_clear(column, row - 1)
                    || !self.hallway_path_is_clear(column, target_column)
                {
                    return None;
                }

                let mut next_state = self.clone();
                next_state.cells.0[0][target_column] = Cell::Occupied(amphipod);
                next_state.cells.0[row][column] = Cell::Empty;

                next_state.energy +=
                    (row + column.abs_diff(target_column)) * amphipod.energy_per_step();
                return Some(next_state);
            }
        }

        None
    }

    fn possible_next_states(&self) -> Vec<Self> {
        let mut next_states = vec![];

        // "Once an amphipod stops moving in the hallway, it will stay in that spot until it can move into a room."
        for j in HALLWAY_COLUMNS_ALLOWED {
            if let Some(next_state) = self.moved_from_hallway_to_room(j) {
                next_states.push(next_state);
            }
        }
        if N == 6 {
            if let Some(next_state) = self.moved_from_hallway_to_room(11) {
                next_states.push(next_state);
            }
        }

        // Amphipods in rooms
        for i in 1..=N {
            for j in ROOM_COLUMNS {
                for target in HALLWAY_COLUMNS_ALLOWED {
                    if let Some(next_state) = self.moved_from_room_to_hallway((i, j), target) {
                        next_states.push(next_state);
                    }
                }
                if N == 6 {
                    if let Some(next_state) = self.moved_from_room_to_hallway((i, j), 11) {
                        next_states.push(next_state);
                    }
                }
            }
        }

        next_states
    }
}

fn unfold(game_state: &GameState<2>) -> GameState<4> {
    let mut cells = [[Cell::Void; 12]; 5];
    for (i, line) in cells.iter_mut().take(2).enumerate() {
        for (j, cell) in line.iter_mut().enumerate() {
            *cell = game_state.cells.0[i][j];
        }
    }
    for j in 0..12 {
        cells[4][j] = game_state.cells.0[2][j];
    }
    cells[2][2] = Cell::Occupied(Amphipod::Desert);
    cells[2][4] = Cell::Occupied(Amphipod::Copper);
    cells[2][6] = Cell::Occupied(Amphipod::Bronze);
    cells[2][8] = Cell::Occupied(Amphipod::Amber);

    cells[3][2] = Cell::Occupied(Amphipod::Desert);
    cells[3][4] = Cell::Occupied(Amphipod::Bronze);
    cells[3][6] = Cell::Occupied(Amphipod::Amber);
    cells[3][8] = Cell::Occupied(Amphipod::Copper);

    for line in cells.iter_mut().skip(2).take(2) {
        for cell in line.iter_mut().skip(1).take(9) {
            if *cell == Cell::Void {
                *cell = Cell::Wall;
            }
        }
    }

    GameState {
        cells: Cells(cells),
        energy: game_state.energy,
    }
}

impl<const N: usize> std::str::FromStr for GameState<N>
where
    [(); (N + 1)]: Sized,
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut initial_amphipods = [[Amphipod::Amber; N]; 4];

        let lines = s.lines().skip(2).take(N);
        for (i, line) in lines.enumerate() {
            let chars = line.chars().skip(1).take(11).collect::<Vec<_>>();
            for (j, col) in ROOM_COLUMNS.iter().enumerate() {
                initial_amphipods[j][i] = Amphipod::from(chars[*col]);
            }
        }

        Ok(Self::new(initial_amphipods))
    }
}

impl<const N: usize> Display for GameState<N>
where
    [(); (N + 1)]: Sized,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "#############")?;
        for i in 0..2 {
            write!(f, "#")?;
            for c in self.cells.0[i].iter().take(11) {
                write!(f, "{}", c)?;
            }
            writeln!(f, "#")?;
        }
        for i in 2..=N {
            write!(f, "  ")?;
            for j in 1..10 {
                write!(f, "{}", self.cells.0[i][j])?;
            }
            writeln!(f)?;
        }
        writeln!(f, "  #########")?;
        Ok(())
    }
}

fn solve_least_amount_of_energy<const N: usize>(state: &GameState<N>) -> usize
where
    [(); (N + 1)]: Sized,
{
    let mut current_min = usize::MAX;
    let mut cache = HashMap::new();
    solved_least_amount_of_energy_internal(state, &mut current_min, &mut cache)
}

fn solved_least_amount_of_energy_internal<const N: usize>(
    state: &GameState<N>,
    current_min: &mut usize,
    cache: &mut HashMap<Cells<N>, usize>,
) -> usize
where
    [(); (N + 1)]: Sized,
{
    // println!("Visiting with energy {}:\n{}", state.energy, state);
    // std::io::stdin().read_exact(&mut [0]).unwrap();
    if state.energy > *current_min {
        return usize::MAX;
    }

    if state.is_solved() {
        if state.energy < *current_min {
            *current_min = state.energy;
            // println!("current least amount of energy is {}", state.energy)
        }
        return state.energy;
    }

    let cached_energy = cache.entry(state.cells.clone()).or_insert(usize::MAX);
    if *cached_energy <= state.energy {
        return usize::MAX;
    } else {
        *cached_energy = state.energy;
    }

    let next_states = state.possible_next_states();
    next_states
        .iter()
        .map(|ns| solved_least_amount_of_energy_internal(ns, current_min, cache))
        .min()
        .unwrap_or(usize::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EXAMPLE1: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
";

    #[test]
    fn part1() {
        let game_state: GameState<2> = INPUT_EXAMPLE1.parse().unwrap();
        assert_eq!(game_state.to_string(), INPUT_EXAMPLE1);

        assert_eq!(solve_least_amount_of_energy(&game_state), 12521);
    }

    const INPUT_EXAMPLE2: &str = "#############
#...........#
###B#C#B#D###
  #D#C#B#A#
  #D#B#A#C#
  #A#D#C#A#
  #########
";

    #[test]
    fn part2() {
        let game_state: GameState<2> = INPUT_EXAMPLE1.parse().unwrap();
        let game_state = unfold(&game_state);
        assert_eq!(game_state.to_string(), INPUT_EXAMPLE2);

        assert_eq!(solve_least_amount_of_energy(&game_state), 44169);
    }

    #[test]
    fn part3() {
        // https://www.reddit.com/r/adventofcode/comments/rn48sp/2021_day_23_part_3/
        let game_state: GameState<6> = INPUT_EXAMPLE3.parse().unwrap();
        assert_eq!(solve_least_amount_of_energy(&game_state), 82849);
    }
}
