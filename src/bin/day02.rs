const INPUT: &str = include_str!("../../inputs/day02.txt");

fn main() {
    let commands = parse_input(INPUT);
    let mut submarine = Submarine::default();
    submarine.navigate(&commands);

    println!(
        "Submarine position 1: {} {}. Answer = {}",
        submarine.horizontal_position,
        submarine.depth,
        submarine.horizontal_position * submarine.depth
    );

    let mut submarine = SubmarineComplex::default();
    submarine.navigate(&commands);

    println!(
        "Submarine position 2: {} {}. Answer = {}",
        submarine.horizontal_position,
        submarine.depth,
        submarine.horizontal_position * submarine.depth
    );
}

#[derive(Debug, PartialEq)]
enum Command {
    Forward(isize),
    Down(isize),
    Up(isize),
}

trait Navigation {
    fn navigate(&mut self, commands: &[Command]);
}

#[derive(Default)]
struct Submarine {
    horizontal_position: isize,
    depth: isize,
}

impl Navigation for Submarine {
    fn navigate(&mut self, commands: &[Command]) {
        for c in commands {
            match c {
                Command::Forward(v) => self.horizontal_position += v,
                Command::Up(v) => self.depth -= v,
                Command::Down(v) => self.depth += v,
            }
        }
    }
}

#[derive(Default)]
struct SubmarineComplex {
    horizontal_position: isize,
    depth: isize,
    aim: isize,
}

impl Navigation for SubmarineComplex {
    fn navigate(&mut self, commands: &[Command]) {
        for c in commands {
            match c {
                Command::Forward(v) => {
                    self.horizontal_position += v;
                    self.depth += self.aim * v;
                }
                Command::Up(v) => self.aim -= v,
                Command::Down(v) => self.aim += v,
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<Command> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

impl std::str::FromStr for Command {
    type Err = ();
    fn from_str(command: &str) -> Result<Self, Self::Err> {
        let (direction, value) = command.split_once(' ').unwrap();
        let value = value.parse().unwrap();
        match direction {
            "forward" => Ok(Command::Forward(value)),
            "down" => Ok(Command::Down(value)),
            "up" => Ok(Command::Up(value)),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EXAMPLE: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn example() {
        let commands = parse_input(INPUT_EXAMPLE);
        assert_eq!(
            commands,
            [
                Command::Forward(5),
                Command::Down(5),
                Command::Forward(8),
                Command::Up(3),
                Command::Down(8),
                Command::Forward(2)
            ]
        );

        // Part 1
        let mut submarine = Submarine::default();
        submarine.navigate(&commands);
        assert_eq!(submarine.horizontal_position, 15);
        assert_eq!(submarine.depth, 10);
        assert_eq!(submarine.horizontal_position * submarine.depth, 150);

        // Part 2
        let mut submarine = SubmarineComplex::default();
        submarine.navigate(&commands);
        assert_eq!(submarine.horizontal_position, 15);
        assert_eq!(submarine.depth, 60);
        assert_eq!(submarine.horizontal_position * submarine.depth, 900);
    }
}
