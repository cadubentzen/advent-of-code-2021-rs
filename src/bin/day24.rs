use std::iter::Peekable;

const INPUT: &str = include_str!("../../inputs/day24.txt");

fn main() {
    let instructions = parse_input(INPUT);
    let mut monad = Monad::new(instructions);

    // This is not actually solving, as the solving was done via inference
    // through the instructions in a spreadsheet ¯\_(ツ)_/¯
    println!("Max:");
    {
        let mut monad = monad.clone();
        for d in [5, 3, 9, 9, 9, 9, 9, 5, 8, 2, 9, 3, 9, 9] {
            let _ = monad.insert_digit(d);
            println!("{:?}", monad.alu);
        }
    }

    println!("Min:");
    for d in [1, 1, 7, 2, 1, 1, 5, 1, 1, 1, 8, 1, 7, 5] {
        let _ = monad.insert_digit(d);
        println!("{:?}", monad.alu);
    }
}

#[derive(Debug, Clone)]
enum Variable {
    W,
    X,
    Y,
    Z,
}

use Variable::*;

impl std::str::FromStr for Variable {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(W),
            "x" => Ok(X),
            "y" => Ok(Y),
            "z" => Ok(Z),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Alu {
    w: isize,
    x: isize,
    y: isize,
    z: isize,
}

impl Alu {
    fn get(&self, var: &Variable) -> isize {
        match var {
            W => self.w,
            X => self.x,
            Y => self.y,
            Z => self.z,
        }
    }

    fn set(&mut self, var: &Variable, value: isize) {
        match var {
            W => self.w = value,
            X => self.x = value,
            Y => self.y = value,
            Z => self.z = value,
        }
    }
}

#[derive(Debug, Clone)]
enum Argument {
    Num(isize),
    Var(Variable),
}

impl std::str::FromStr for Argument {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(var) = s.parse::<Variable>() {
            Ok(Argument::Var(var))
        } else {
            Ok(Argument::Num(s.parse().unwrap()))
        }
    }
}

impl Argument {
    fn resolve(&self, alu: &Alu) -> isize {
        match self {
            Argument::Num(num) => *num,
            Argument::Var(W) => alu.w,
            Argument::Var(X) => alu.x,
            Argument::Var(Y) => alu.y,
            Argument::Var(Z) => alu.z,
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Inp(Variable),
    Add(Variable, Argument),
    Mul(Variable, Argument),
    Div(Variable, Argument),
    Mod(Variable, Argument),
    Eql(Variable, Argument),
}

use Instruction::*;

impl std::str::FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let instruction = match parts.next().unwrap() {
            "inp" => Inp(parts.next().unwrap().parse().unwrap()),
            "add" => Add(
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            ),
            "mul" => Mul(
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            ),
            "div" => Div(
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            ),
            "mod" => Mod(
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            ),
            "eql" => Eql(
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            ),
            _ => unreachable!(),
        };
        Ok(instruction)
    }
}

impl Instruction {
    fn run(&self, alu: &mut Alu) -> Result<(), MonadErr> {
        match self {
            Inp(_) => unreachable!(),
            Add(var, val) => alu.set(var, alu.get(var) + val.resolve(alu)),
            Mul(var, val) => alu.set(var, alu.get(var) * val.resolve(alu)),
            Div(var, val) => {
                let b = val.resolve(alu);
                if b == 0 {
                    return Err(MonadErr::Crash);
                }
                alu.set(var, alu.get(var) / b);
            }
            Mod(var, val) => {
                let a = alu.get(var);
                let b = val.resolve(alu);
                if a < 0 || b <= 0 {
                    return Err(MonadErr::Crash);
                }
                alu.set(var, a % b);
            }
            Eql(var, val) => {
                let a = alu.get(var);
                let b = val.resolve(alu);
                alu.set(var, if a == b { 1 } else { 0 });
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Monad {
    alu: Alu,
    instructions: Peekable<std::vec::IntoIter<Instruction>>,
}

impl Monad {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            alu: Alu::default(),
            instructions: instructions.into_iter().peekable(),
        }
    }
}

enum ModelNumberStatus {
    Valid,
    Invalid,
}

enum MonadErr {
    NeedsInput,
    Crash,
}

impl Monad {
    fn model_number_status(&mut self) -> ModelNumberStatus {
        assert!(self.instructions.peek().is_none());
        if self.alu.z == 0 {
            ModelNumberStatus::Valid
        } else {
            ModelNumberStatus::Invalid
        }
    }

    fn insert_digit(&mut self, input: u8) -> Result<ModelNumberStatus, MonadErr> {
        let instruction = self.instructions.next();
        match instruction {
            Some(Instruction::Inp(var)) => self.alu.set(&var, input as isize),
            _ => unreachable!("run() should be called only when the next instruction is inp"),
        }

        while !matches!(self.instructions.peek(), Some(Instruction::Inp(_))) {
            if let Some(instruction) = self.instructions.next() {
                instruction.run(&mut self.alu)?;
            } else {
                // println!("ran until the end!");
                return Ok(self.model_number_status());
            }
        }

        Err(MonadErr::NeedsInput)
    }
}

fn parse_input(s: &str) -> Vec<Instruction> {
    s.lines().map(|line| line.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EXAMPLE1: &str = "inp x
mul x -1
";
    const INPUT_EXAMPLE2: &str = "inp z
inp x
mul z 3
eql z x
";

    const INPUT_EXAMPLE3: &str = "inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2
";

    #[test]
    fn example() {
        let instructions = parse_input(INPUT_EXAMPLE1);
        dbg!(instructions);

        let instructions = parse_input(INPUT_EXAMPLE2);
        dbg!(instructions);

        let instructions = parse_input(INPUT_EXAMPLE3);
        dbg!(instructions);
    }
}
