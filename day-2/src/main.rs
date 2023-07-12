const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;
const LOST: u32 = 0;
const DRAW: u32 = 3;
const WIN: u32 = 6;

struct Round {
    player_move: Move,
    opponent_move: Move,
    outcome: Outcome,
}

enum Move {
    Rock,
    Paper,
    Scissors
}

enum Outcome {
    Win,
    Lose,
    Draw
}

fn main() {

    let playbook: Vec<&str> = include_str!("./input.txt").lines().collect();

    let mut round_outcome: Vec<char> = Vec::new();
    let mut opponent_move: Vec<char> = Vec::new();
    let mut rounds: Vec<Round> = Vec::new();

    for play in playbook {
        let round: Vec<&str> = play.split(" ").collect();
        let mut opponent_move: Move = Move::Rock;
        let mut player_move: Move = Move::Rock;

        if round[0].chars().as_str() == "A" {
            opponent_move = Move::Rock;
        } else if round[0].chars().as_str() == "B" {
            opponent_move = Move::Paper;
        } else if round[0].chars().as_str() == "C" {
            opponent_move = Move::Scissors;
        }

        if round[1].chars().as_str() == "X" {
            player_move = Move::Rock;
        } else if round[1].chars().as_str() == "Y" {
            player_move = Move::Paper;
        } else if round[1].chars().as_str() == "Z" {
            player_move = Move::Scissors;
        }

        rounds.push(Round {
            player_move,
            opponent_move,
            outcome: Outcome::Lose
        });
    }

    let mut score: u32 = 0;
    round_outcome.iter().enumerate().for_each(|(i, round_move)| {
        // both players choose rock
        if *round_move == 'x' && *opponent_move.get(i).unwrap() == 'a' {
            rounds.push(Round {
                player_move: Move::Rock,
                opponent_move: Move::Rock,
                outcome: Outcome::Draw
            });
        }
        // Both players chose Paper
        if *round_move == 'Y' && *opponent_move.get(i).unwrap() == 'B' {
            rounds.push(Round {
                player_move: Move::Paper,
                opponent_move: Move::Paper,
                outcome: Outcome::Draw
            });
        }
        // Both players chose scissors
        if *round_move == 'Z' && *opponent_move.get(i).unwrap() == 'C' {
            rounds.push(Round {
                player_move: Move::Scissors,
                opponent_move: Move::Scissors,
                outcome: Outcome::Draw
            });
            score += DRAW + SCISSORS;
        }
        // I chose rock, oppenent chose paper
        if *round_move == 'X' && *opponent_move.get(i).unwrap() == 'B' {
            rounds.push(Round {
                player_move: Move::Scissors,
                opponent_move: Move::Scissors,
                outcome: Outcome::Draw
            });
            score += LOST + ROCK;
        }
        // I chose rock, opponent chose scissors
        if *round_move == 'X' && *opponent_move.get(i).unwrap() == 'C' {
            rounds.push(Round {
                player_move: Move::Scissors,
                opponent_move: Move::Scissors,
                outcome: Outcome::Draw
            });
            score += WIN + ROCK;
        }
        // I chose paper, opponent chose rock
        if *round_move == 'Y' && *opponent_move.get(i).unwrap() == 'A' {
            rounds.push(Round {
                player_move: Move::Scissors,
                opponent_move: Move::Scissors,
                outcome: Outcome::Draw
            });
            score += WIN + PAPER;
        }
        // I chose paper, opponent chose scissors
        if *round_move == 'Y' && *opponent_move.get(i).unwrap() == 'C' {
            rounds.push(Round {
                player_move: Move::Scissors,
                opponent_move: Move::Scissors,
                outcome: Outcome::Draw
            });
            score += LOST + PAPER;
        }
        // I chose scissors, opponent chose rock
        if *round_move == 'Z' && *opponent_move.get(i).unwrap() == 'A' {
            rounds.push(Round {
                player_move: Move::Scissors,
                opponent_move: Move::Scissors,
                outcome: Outcome::Draw
            });
            score += LOST + SCISSORS;
        }
        // I chose scissors, opponent chose paper
        if *round_move == 'Z' && *opponent_move.get(i).unwrap() == 'B' {
            rounds.push(Round {
                player_move: Move::Scissors,
                opponent_move: Move::Scissors,
                outcome: Outcome::Draw
            });
            score += WIN + SCISSORS;
        }
    });


    println!("Following the strategy guide, I get a score of: {score}");

    let mut score: u32 = 0;
    round_outcome.iter().enumerate().for_each(|(i, round_outcome)| {
        // Both players choose Rock
        if *round_outcome == 'X' && *opponent_move.get(i).unwrap() == 'A' {
            score += DRAW + ROCK;
        }
        // Both players chose Paper
        if *round_outcome == 'Y' && *opponent_move.get(i).unwrap() == 'B' {
            score += DRAW + PAPER;
        }
        // Both players chose scissors
        if *round_outcome == 'Z' && *opponent_move.get(i).unwrap() == 'C' {
            score += DRAW + SCISSORS;
        }
        // I chose rock, oppenent chose paper
        if *round_outcome == 'X' && *opponent_move.get(i).unwrap() == 'B' {
            score += LOST + ROCK;
        }
        // I chose rock, opponent chose scissors
        if *round_outcome == 'X' && *opponent_move.get(i).unwrap() == 'C' {
            score += WIN + ROCK;
        }
        // I chose paper, opponent chose rock
        if *round_outcome == 'Y' && *opponent_move.get(i).unwrap() == 'A' {
            score += WIN + PAPER;
        }
        // I chose paper, opponent chose scissors
        if *round_outcome == 'Y' && *opponent_move.get(i).unwrap() == 'C' {
            score += LOST + PAPER;
        }
        // I chose scissors, opponent chose rock
        if *round_outcome == 'Z' && *opponent_move.get(i).unwrap() == 'A' {
            score += LOST + SCISSORS;
        }
        // I chose scissors, opponent chose paper
        if *round_outcome == 'Z' && *opponent_move.get(i).unwrap() == 'B' {
            score += WIN + SCISSORS;
        }
    });
}
