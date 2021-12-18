use nom::{
    character::complete::{char, digit1},
    combinator::peek,
    IResult,
};
use std::fmt::Display;

const INPUT: &str = include_str!("../../inputs/day18.txt");

fn main() {
    let numbers = parse_input(INPUT);
    let sum = sum_all(numbers.clone());
    println!("Answer 1: {}", sum.magnitude());
    let largest = largest_magnitude_of_two(numbers);
    println!("Answer 2: {}", largest);
}

#[derive(Debug, PartialEq, Clone)]
struct SnailfishNumber(Box<Element>, Box<Element>);

#[derive(Debug, PartialEq, Clone)]
enum Element {
    Number(u8),
    Pair(Box<Element>, Box<Element>),
}

fn parse_input(s: &str) -> Vec<SnailfishNumber> {
    s.lines().map(SnailfishNumber::new).collect()
}

fn parse_element(s: &str) -> IResult<&str, Element> {
    if let Ok((s, _)) = peek(digit1::<_, ()>)(s) {
        let (s, value) = digit1(s)?;
        Ok((s, Element::Number(value.parse().unwrap())))
    } else {
        let (s, (elem1, elem2)) = parse_pair(s)?;
        Ok((s, Element::Pair(Box::from(elem1), Box::from(elem2))))
    }
}

fn parse_pair(s: &str) -> IResult<&str, (Element, Element)> {
    let (s, _) = char('[')(s)?;
    let (s, elem1) = parse_element(s)?;
    let (s, _) = char(',')(s)?;
    let (s, elem2) = parse_element(s)?;
    let (s, _) = char(']')(s)?;

    Ok((s, (elem1, elem2)))
}

enum ReduceOp {
    Explode,
    Split,
}

impl SnailfishNumber {
    fn new(s: &str) -> Self {
        Self::parse(s).unwrap().1
    }

    fn parse(s: &str) -> IResult<&str, SnailfishNumber> {
        let (s, (left, right)) = parse_pair(s)?;
        Ok((s, SnailfishNumber(Box::from(left), Box::from(right))))
    }

    fn reduce(&mut self) {
        while self.reduce_step().is_some() {}
    }

    fn reduce_step(&mut self) -> Option<ReduceOp> {
        if self.explode() {
            Some(ReduceOp::Explode)
        } else if self.split() {
            Some(ReduceOp::Split)
        } else {
            None
        }
    }

    fn explode(&mut self) -> bool {
        self.0.explode(1, None, Some(self.1.leftmost()))
            || self.1.explode(1, Some(self.0.rightmost()), None)
    }

    fn split(&mut self) -> bool {
        self.0.split() || self.1.split()
    }

    fn add(self, other: Self) -> Self {
        let mut sum = Self(
            Box::from(Element::Pair(self.0, self.1)),
            Box::from(Element::Pair(other.0, other.1)),
        );
        sum.reduce();
        sum
    }

    fn magnitude(&self) -> usize {
        3 * self.0.magnitude() + 2 * self.1.magnitude()
    }
}

fn sum_all(numbers: Vec<SnailfishNumber>) -> SnailfishNumber {
    let mut sum: Option<SnailfishNumber> = None;
    for num in numbers.into_iter() {
        match sum {
            Some(s) => sum = Some(s.add(num)),
            None => sum = Some(num),
        }
    }
    sum.unwrap()
}

fn largest_magnitude_of_two(numbers: Vec<SnailfishNumber>) -> usize {
    let mut largest = 0;
    for left in &numbers {
        for right in &numbers {
            if left == right {
                continue;
            }
            let sum = left.clone().add(right.clone());
            largest = largest.max(sum.magnitude());
        }
    }
    largest
}

impl Element {
    fn leftmost(&mut self) -> &mut u8 {
        match self {
            Element::Number(value) => value,
            Element::Pair(left, _) => left.leftmost(),
        }
    }

    fn rightmost(&mut self) -> &mut u8 {
        match self {
            Element::Number(value) => value,
            Element::Pair(_, right) => right.rightmost(),
        }
    }

    fn explode(
        &mut self,
        level: usize,
        left_value: Option<&mut u8>,
        right_value: Option<&mut u8>,
    ) -> bool {
        if level == 4 {
            match self {
                Element::Number(_) => false,
                Element::Pair(left, right) => {
                    if let Element::Number(left) = left.as_ref() {
                        if let Some(left_value) = left_value {
                            *left_value += left;
                        }
                    } else {
                        unreachable!()
                    }

                    if let Element::Number(right) = right.as_ref() {
                        if let Some(right_value) = right_value {
                            *right_value += right;
                        }
                    } else {
                        unreachable!()
                    }

                    *self = Element::Number(0);

                    true
                }
            }
        } else {
            match self {
                Element::Number(_) => false,
                Element::Pair(left, right) => {
                    left.explode(level + 1, left_value, Some(right.leftmost()))
                        || right.explode(level + 1, Some(left.rightmost()), right_value)
                }
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Element::Number(value) => {
                if *value >= 10 {
                    let left = *value / 2;
                    let right = *value / 2 + *value % 2;
                    *self = Element::Pair(
                        Box::from(Element::Number(left)),
                        Box::from(Element::Number(right)),
                    );
                    true
                } else {
                    false
                }
            }
            Element::Pair(left, right) => left.split() || right.split(),
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            Element::Number(value) => *value as usize,
            Element::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Number(value) => write!(f, "{}", value),
            Element::Pair(elem1, elem2) => write!(f, "[{},{}]", elem1, elem2),
        }
    }
}

impl Display for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.0, self.1)
    }
}

impl std::str::FromStr for SnailfishNumber {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SnailfishNumber::parse(s).unwrap().1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        // Let's check that Display is properly implemented with basic examples.
        // It will make it easier to assert the parsing
        assert_eq!(
            SnailfishNumber(Box::from(Element::Number(1)), Box::from(Element::Number(2)))
                .to_string(),
            "[1,2]"
        );

        assert_eq!(
            SnailfishNumber(
                Box::from(Element::Pair(
                    Box::new(Element::Number(1)),
                    Box::new(Element::Number(2))
                )),
                Box::from(Element::Number(3))
            )
            .to_string(),
            "[[1,2],3]"
        );

        // Now let's test parsing
        let inputs = [
            "[1,2]",
            "[[1,2],3]",
            "[9,[8,7]]",
            "[[1,9],[8,5]]",
            "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]",
            "[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]",
            "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]",
        ];
        for input in inputs {
            // parse it
            let sfn = input.parse::<SnailfishNumber>().unwrap();
            // convert it back to string and check
            assert_eq!(sfn.to_string(), input);
        }
    }

    #[test]
    fn left_rightmost() {
        let mut sfn: SnailfishNumber = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".parse().unwrap();
        assert_eq!(sfn.0.leftmost(), &3);
        assert_eq!(sfn.0.rightmost(), &3);
        assert_eq!(sfn.1.leftmost(), &6);
        assert_eq!(sfn.1.rightmost(), &2);
    }

    #[test]
    fn explode() {
        let examples = [
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
        ];

        for (input, output) in examples {
            let mut sfn: SnailfishNumber = input.parse().unwrap();
            assert!(sfn.explode());
            assert_eq!(sfn.to_string(), output);
        }
    }

    #[test]
    fn split() {
        let mut sfn: SnailfishNumber = "[[[[0,7],4],[15,[0,13]]],[1,1]]".parse().unwrap();
        assert!(sfn.split());
        assert_eq!(sfn.to_string(), "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
        assert!(sfn.split());
        assert_eq!(sfn.to_string(), "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");
    }

    #[test]
    fn reduce() {
        let mut sfn: SnailfishNumber = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".parse().unwrap();
        assert!(matches!(sfn.reduce_step(), Some(ReduceOp::Explode)));
        assert_eq!(sfn.to_string(), "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]");
        assert!(matches!(sfn.reduce_step(), Some(ReduceOp::Explode)));
        assert_eq!(sfn.to_string(), "[[[[0,7],4],[15,[0,13]]],[1,1]]");
        assert!(matches!(sfn.reduce_step(), Some(ReduceOp::Split)));
        assert_eq!(sfn.to_string(), "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
        assert!(matches!(sfn.reduce_step(), Some(ReduceOp::Split)));
        assert_eq!(sfn.to_string(), "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");
        assert!(matches!(sfn.reduce_step(), Some(ReduceOp::Explode)));
        assert_eq!(sfn.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        assert!(matches!(sfn.reduce_step(), None));
    }

    #[test]
    fn add() {
        let mut sum = SnailfishNumber::new("[1,1]")
            .add("[2,2]".parse().unwrap())
            .add("[3,3]".parse().unwrap())
            .add("[4,4]".parse().unwrap());
        assert_eq!(sum.to_string(), "[[[[1,1],[2,2]],[3,3]],[4,4]]");
        sum = sum.add("[5,5]".parse().unwrap());
        assert_eq!(sum.to_string(), "[[[[3,0],[5,3]],[4,4]],[5,5]]");

        let larger_example = [
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]",
        ];
        let mut sum = SnailfishNumber::new(larger_example[0]);
        for sfn in larger_example.iter().skip(1) {
            sum = sum.add(SnailfishNumber::new(sfn));
        }
        assert_eq!(
            sum.to_string(),
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
        );
    }

    #[test]
    fn magnitude() {
        let examples = [
            ("[9,1]", 29),
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            (
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
                3488,
            ),
        ];

        for (input, magnitude) in examples {
            assert_eq!(SnailfishNumber::new(input).magnitude(), magnitude);
        }
    }

    const HOMEWORK_ASSIGNMENT: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
";

    #[test]
    fn homework_assignment() {
        let numbers = parse_input(HOMEWORK_ASSIGNMENT);
        let sum = sum_all(numbers);
        assert_eq!(
            sum.to_string(),
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
        );
        assert_eq!(sum.magnitude(), 4140);
    }

    #[test]
    fn largest() {
        let numbers = parse_input(HOMEWORK_ASSIGNMENT);
        let largest = largest_magnitude_of_two(numbers);
        assert_eq!(largest, 3993);
    }
}
