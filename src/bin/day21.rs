use std::collections::HashMap;

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day21.txt");

fn main() {
    let (player1, player2) = parse_input(INPUT);
    println!("Answer 1: {}", play_game(player1.clone(), player2.clone()));
    let (win1, win2) = play_dirac_game(player1, player2);
    println!("Answer 2: {}", win1.max(win2));
}

#[derive(Debug, Clone)]
struct Player {
    position: u8,
    score: u16,
}

fn advance_position(position: u8, dice_value: u16) -> u8 {
    let mut position = (position as u16 + dice_value) % 10;
    if position == 0 {
        position = 10;
    }
    position as u8
}

fn play_turn(player_in_turn: &mut Player, other: &mut Player, dice_values: &[u8]) -> Option<usize> {
    player_in_turn.position = advance_position(
        player_in_turn.position,
        dice_values.iter().map(|n| *n as u16).sum::<u16>(),
    );
    player_in_turn.score += player_in_turn.position as u16;
    if player_in_turn.score >= 1000 {
        return Some(other.score as usize);
    }
    None
}

fn play_game(mut player1: Player, mut player2: Player) -> usize {
    let mut turn = true;
    for (i, dice_values) in (1..=100).cycle().chunks(3).into_iter().enumerate() {
        let dice_values = dice_values.collect::<Vec<_>>();

        let loser_score = if turn {
            play_turn(&mut player1, &mut player2, &dice_values)
        } else {
            play_turn(&mut player2, &mut player1, &dice_values)
        };
        if let Some(loser_score) = loser_score {
            return loser_score * (i + 1) * 3;
        }
        turn = !turn;
    }
    unreachable!()
}

fn play_dirac_round(
    score_position_in_turn: &mut HashMap<(u8, u8, u8, u8), u64>,
    other: &mut HashMap<(u8, u8, u8, u8), u64>,
) -> u64 {
    let mut num_wins = 0;
    for ((position1, score1, position2, score2), quantity) in score_position_in_turn.iter() {
        for ((d1, d2), d3) in (1..=3).cartesian_product(1..=3).cartesian_product(1..=3) {
            let new_position1 = advance_position(*position1, d1 + d2 + d3);
            let new_score1 = score1 + new_position1;

            if new_score1 >= 21 {
                num_wins += *quantity;
            } else {
                *other
                    .entry((*position2, *score2, new_position1, new_score1))
                    .or_default() += *quantity;
            }
        }
    }
    score_position_in_turn.clear();
    num_wins
}

fn play_dirac_game(player1: Player, player2: Player) -> (u64, u64) {
    let mut score_position_turn1 = HashMap::<(u8, u8, u8, u8), u64>::new();
    let mut score_position_turn2 = HashMap::<(u8, u8, u8, u8), u64>::new();
    let mut num_wins1 = 0;
    let mut num_wins2 = 0;

    score_position_turn1.insert((player1.position, 0, player2.position, 0), 1);

    while !score_position_turn1.is_empty() {
        num_wins1 += play_dirac_round(&mut score_position_turn1, &mut score_position_turn2);
        num_wins2 += play_dirac_round(&mut score_position_turn2, &mut score_position_turn1)
    }

    (num_wins1, num_wins2)
}

fn parse_input(s: &str) -> (Player, Player) {
    let map_to_player = |line: &str| Player {
        position: line.trim().split(' ').last().unwrap().parse().unwrap(),
        score: 0,
    };

    let (player1, player2) = s.split_once('\n').unwrap();
    (map_to_player(player1), map_to_player(player2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EXAMPLE: &str = "Player 1 starting position: 4
Player 2 starting position: 8
";

    #[test]
    fn example() {
        let (player1, player2) = parse_input(INPUT_EXAMPLE);

        assert_eq!(play_game(player1.clone(), player2.clone()), 739785);

        assert_eq!(
            play_dirac_game(player1, player2),
            (444356092776315, 341960390180808),
        );
    }
}
