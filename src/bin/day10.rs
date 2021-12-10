const INPUT: &str = include_str!("../../inputs/day10.txt");

fn main() {
    let lines = parse_input(INPUT);
    println!("Answer 1: {}", total_syntax_error_score(&lines));
    println!("Answer 2: {}", middle_score_incomplete_lines(&lines));
}

fn parse_input(input: &str) -> Vec<Vec<Bracket>> {
    input
        .lines()
        .map(|line| line.chars().map(Bracket::from).collect())
        .collect()
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Bracket {
    op: BracketOp,
    shape: BracketShape,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum BracketOp {
    Open,
    Close,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum BracketShape {
    Round,
    Square,
    Curly,
    Arrow,
}

impl From<char> for Bracket {
    fn from(c: char) -> Self {
        match c {
            '(' => Bracket {
                op: BracketOp::Open,
                shape: BracketShape::Round,
            },
            ')' => Bracket {
                op: BracketOp::Close,
                shape: BracketShape::Round,
            },
            '[' => Bracket {
                op: BracketOp::Open,
                shape: BracketShape::Square,
            },
            ']' => Bracket {
                op: BracketOp::Close,
                shape: BracketShape::Square,
            },
            '{' => Bracket {
                op: BracketOp::Open,
                shape: BracketShape::Curly,
            },
            '}' => Bracket {
                op: BracketOp::Close,
                shape: BracketShape::Curly,
            },
            '<' => Bracket {
                op: BracketOp::Open,
                shape: BracketShape::Arrow,
            },
            '>' => Bracket {
                op: BracketOp::Close,
                shape: BracketShape::Arrow,
            },
            _ => unreachable!(),
        }
    }
}

impl From<&Bracket> for char {
    fn from(b: &Bracket) -> Self {
        match b.op {
            BracketOp::Open => b.shape.opening_char(),
            BracketOp::Close => b.shape.closing_char(),
        }
    }
}

impl BracketShape {
    #[inline]
    fn opening_char(&self) -> char {
        match self {
            BracketShape::Round => '(',
            BracketShape::Square => '[',
            BracketShape::Curly => '{',
            BracketShape::Arrow => '<',
        }
    }

    #[inline]
    fn closing_char(&self) -> char {
        match self {
            BracketShape::Round => ')',
            BracketShape::Square => ']',
            BracketShape::Curly => '}',
            BracketShape::Arrow => '>',
        }
    }
}

impl PartialEq<char> for Bracket {
    fn eq(&self, other: &char) -> bool {
        &char::from(self) == other
    }
}

enum LineType {
    Corrupt(BracketShape),
    Incomplete(Vec<BracketShape>),
}

fn analyze(brackets: &[Bracket]) -> LineType {
    let mut stack = vec![];
    for b in brackets {
        match b.op {
            BracketOp::Open => stack.push(b.shape),
            BracketOp::Close => {
                let last = stack.last().unwrap();
                if &b.shape != last {
                    return LineType::Corrupt(b.shape);
                }
                stack.pop();
            }
        }
    }
    // No lines are complete as per problem statement
    assert!(!stack.is_empty());
    LineType::Incomplete(stack)
}

fn syntax_error_score(brackets: &[Bracket]) -> Option<usize> {
    if let LineType::Corrupt(shape) = analyze(brackets) {
        let score = match shape {
            BracketShape::Round => 3,
            BracketShape::Square => 57,
            BracketShape::Curly => 1197,
            BracketShape::Arrow => 25137,
        };
        Some(score)
    } else {
        None
    }
}

fn total_syntax_error_score(lines: &[Vec<Bracket>]) -> usize {
    lines
        .iter()
        .filter_map(|line| syntax_error_score(line))
        .sum()
}

fn completion_score(brackets: &[Bracket]) -> Option<usize> {
    if let LineType::Incomplete(pending) = analyze(brackets) {
        let score = pending.iter().rev().fold(0, |acc, s| {
            let points = match s {
                BracketShape::Round => 1,
                BracketShape::Square => 2,
                BracketShape::Curly => 3,
                BracketShape::Arrow => 4,
            };
            5 * acc + points
        });
        Some(score)
    } else {
        None
    }
}

fn middle_score_incomplete_lines(lines: &[Vec<Bracket>]) -> usize {
    let mut scores = lines
        .iter()
        .filter_map(|line| completion_score(line))
        .collect::<Vec<_>>();

    let len = scores.len();
    // "There will always be an odd number of scores to consider."
    assert!(len % 2 != 0);
    scores.select_nth_unstable(len / 2);
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EXAMPLE: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";

    #[test]
    fn example() {
        let lines = parse_input(INPUT_EXAMPLE);
        assert_eq!(total_syntax_error_score(&lines), 26397);
        assert_eq!(middle_score_incomplete_lines(&lines), 288957);
    }
}
