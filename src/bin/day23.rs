#![feature(int_abs_diff)]

use std::{fmt::Display, hash::Hash};

const INPUT: &str = include_str!("../../inputs/day23.txt");

fn main() {
    let game_state: GameState = INPUT.parse().unwrap();
    println!("Answer 1: {}", solved_least_amount_of_energy(&game_state));
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
    fn from(c: char) -> Self {
        match c {
            'A' => Self::Amber,
            'B' => Self::Bronze,
            'C' => Self::Copper,
            'D' => Self::Desert,
            _ => unreachable!(),
        }
    }

    fn column(&self) -> usize {
        match self {
            Self::Amber => 2,
            Self::Bronze => 4,
            Self::Copper => 6,
            Self::Desert => 8,
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
struct Cells([[Cell; 11]; 3]);

#[derive(Debug, Clone)]
struct GameState {
    cells: Cells,
    energy: usize,
}

impl Ord for GameState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.energy.cmp(&self.energy)
    }
}

impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.energy == other.energy
    }
}

impl Eq for GameState {}

const ROOM_COLUMNS: [usize; 4] = [2, 4, 6, 8];
const HALLWAY_COLUMNS_ALLOWED: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];

impl GameState {
    fn new(initial_amphipods: [Amphipod; 8]) -> Self {
        let mut cells = [[Cell::Void; 11]; 3];

        for cell in &mut cells[0] {
            *cell = Cell::Empty;
        }

        cells[1][0] = Cell::Wall;
        cells[1][1] = Cell::Wall;
        cells[1][3] = Cell::Wall;
        cells[2][1] = Cell::Wall;
        cells[2][3] = Cell::Wall;
        cells[1][5] = Cell::Wall;
        cells[2][5] = Cell::Wall;
        cells[1][7] = Cell::Wall;
        cells[2][7] = Cell::Wall;
        cells[1][9] = Cell::Wall;
        cells[2][9] = Cell::Wall;
        cells[1][10] = Cell::Wall;

        cells[1][2] = Cell::Occupied(initial_amphipods[0]);
        cells[2][2] = Cell::Occupied(initial_amphipods[1]);
        cells[1][4] = Cell::Occupied(initial_amphipods[2]);
        cells[2][4] = Cell::Occupied(initial_amphipods[3]);
        cells[1][6] = Cell::Occupied(initial_amphipods[4]);
        cells[2][6] = Cell::Occupied(initial_amphipods[5]);
        cells[1][8] = Cell::Occupied(initial_amphipods[6]);
        cells[2][8] = Cell::Occupied(initial_amphipods[7]);

        Self {
            cells: Cells(cells),
            energy: 0,
        }
    }

    /// Solved in the following configuration:
    /// ...........
    /// ##A#B#C#D##
    ///  #A#B#C#D#
    fn is_solved(&self) -> bool {
        self.cells.0[1][2] == Cell::Occupied(Amphipod::Amber)
            && self.cells.0[2][2] == Cell::Occupied(Amphipod::Amber)
            && self.cells.0[1][4] == Cell::Occupied(Amphipod::Bronze)
            && self.cells.0[2][4] == Cell::Occupied(Amphipod::Bronze)
            && self.cells.0[1][6] == Cell::Occupied(Amphipod::Copper)
            && self.cells.0[2][6] == Cell::Occupied(Amphipod::Copper)
            && self.cells.0[1][8] == Cell::Occupied(Amphipod::Desert)
            && self.cells.0[2][8] == Cell::Occupied(Amphipod::Desert)
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

    fn moved_from_hallway_to_room(&self, column: usize) -> Option<Self> {
        if let Cell::Occupied(amphipod) = self.cells.0[0][column] {
            let target_column = amphipod.column();
            assert!(column != target_column);

            // Check the path to column is empty
            if !self.hallway_path_is_clear(column, target_column) {
                return None;
            }

            // "Amphipods will never move from the hallway into a room unless
            // that room is their destination room and that room contains no
            // amphipods which do not also have that room as their own
            // destination. If an amphipod's starting room is not its
            // destination room, it can stay in that room until it leaves the
            // room."
            // Try and move to either:
            // 1. Empty column it belongs to - in this case move to the bottom of the column
            if self.cells.0[1][target_column] == Cell::Empty
                && self.cells.0[2][target_column] == Cell::Empty
            {
                let mut next_state = self.clone();
                next_state.cells.0[2][target_column] = Cell::Occupied(amphipod);
                next_state.cells.0[0][column] = Cell::Empty;

                next_state.energy +=
                    (2 + column.abs_diff(target_column)) * amphipod.energy_per_step();
                return Some(next_state);
            }
            // 2. Column with another one of its kind - in this case move to the top of the column
            else if self.cells.0[1][target_column] == Cell::Empty
                && self.cells.0[2][target_column] == Cell::Occupied(amphipod)
            {
                let mut next_state = self.clone();
                next_state.cells.0[1][target_column] = Cell::Occupied(amphipod);
                next_state.cells.0[0][column] = Cell::Empty;

                next_state.energy +=
                    (1 + column.abs_diff(target_column)) * amphipod.energy_per_step();
                return Some(next_state);
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
                if ((row == 2
                    && (!matches!(self.cells.0[1][column], Cell::Empty) || column == final_column))
                    || (row == 1
                        && column == final_column
                        && self.cells.0[2][column] == Cell::Occupied(amphipod)))
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

        // Amphipods in rooms
        for i in [1, 2] {
            for j in ROOM_COLUMNS {
                for target in HALLWAY_COLUMNS_ALLOWED {
                    if let Some(next_state) = self.moved_from_room_to_hallway((i, j), target) {
                        next_states.push(next_state);
                    }
                }
            }
        }

        next_states
    }
}

impl std::str::FromStr for GameState {
    type Err = ();

    // #############
    // #...........#
    // ###B#C#B#D###
    //   #A#D#C#A#
    //   #########
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut initial_amphipods = [Amphipod::Amber; 8];

        let mut lines = s.lines().skip(2);
        let chars = lines.next().unwrap().chars().collect::<Vec<_>>();

        initial_amphipods[0] = Amphipod::from(chars[3]);
        initial_amphipods[2] = Amphipod::from(chars[5]);
        initial_amphipods[4] = Amphipod::from(chars[7]);
        initial_amphipods[6] = Amphipod::from(chars[9]);

        let chars = lines.next().unwrap().chars().collect::<Vec<_>>();
        initial_amphipods[1] = Amphipod::from(chars[3]);
        initial_amphipods[3] = Amphipod::from(chars[5]);
        initial_amphipods[5] = Amphipod::from(chars[7]);
        initial_amphipods[7] = Amphipod::from(chars[9]);

        Ok(Self::new(initial_amphipods))
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "#############")?;
        for i in 0..2 {
            write!(f, "#")?;
            for c in self.cells.0[i] {
                write!(f, "{}", c)?;
            }
            writeln!(f, "#")?;
        }
        write!(f, "  ")?;
        for i in 1..10 {
            write!(f, "{}", self.cells.0[2][i])?;
        }
        writeln!(f)?;
        writeln!(f, "  #########")?;
        Ok(())
    }
}

impl Amphipod {
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

fn solved_least_amount_of_energy(state: &GameState) -> usize {
    let mut current_min = usize::MAX;
    solved_least_amount_of_energy_internal(state, &mut current_min)
}

fn solved_least_amount_of_energy_internal(state: &GameState, current_min: &mut usize) -> usize {
    if state.energy > *current_min {
        return usize::MAX;
    }

    if state.is_solved() {
        if state.energy < *current_min {
            *current_min = state.energy;
            println!("current least amount of energy is {}", state.energy)
        }
        return state.energy;
    }

    let next_states = state.possible_next_states();
    next_states
        .iter()
        .map(|ns| solved_least_amount_of_energy_internal(ns, current_min))
        .min()
        .unwrap_or(usize::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EXAMPLE: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
";

    #[test]
    fn example() {
        let game_state: GameState = INPUT_EXAMPLE.parse().unwrap();
        assert_eq!(game_state.to_string(), INPUT_EXAMPLE);

        assert_eq!(solved_least_amount_of_energy(&game_state), 12521);
    }
}
