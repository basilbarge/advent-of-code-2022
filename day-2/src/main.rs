const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;
const LOST: u32 = 0;
const DRAW: u32 = 3;
const WIN: u32 = 6;

struct Round {
    player_move: char,
    opponent_move: char,
    outcome: bool,
}

fn main() {

    let playbook: Vec<&str> = include_str!("./input.txt").lines().collect();

    let mut my_move: Vec<&str> = Vec::new();
    let mut opponent_move: Vec<&str> = Vec::new();

    for play in playbook {
        let round = play.split(" ").collect::<Vec<char>>();

        opponent_move.push(round[0]);
        my_move.push(round[1]);
    }

    let mut score: u32 = 0;
    my_move.iter().enumerate().for_each(|(i, round_move)| {
        // Both players choose Rock
        if round_move == 'X' && opponent_move.get(i).unwrap() == 'A' {
            score += DRAW + ROCK;
        }
        // Both players chose Paper
        if round_move == 'Y' && opponent_move.get(i).unwrap() == 'B' {
            score += DRAW + PAPER;
        }
        // Both players chose scissors
        if round_move == 'Z' && opponent_move.get(i).unwrap() == 'C' {
            score += DRAW + SCISSORS;
        }
        // I chose rock, oppenent chose paper
        if round_move == 'X' && opponent_move.get(i).unwrap() == 'B' {
            score += LOST + ROCK;
        }
        // I chose rock, opponent chose scissors
        if round_move == 'X' && opponent_move.get(i).unwrap() == 'C' {
            score += WIN + ROCK;
        }
        // I chose paper, opponent chose rock
        if round_move == 'Y' && opponent_move.get(i).unwrap() == 'A' {
            score += WIN + PAPER;
        }
        // I chose paper, opponent chose scissors
        if round_move == 'Y' && opponent_move.get(i).unwrap() == 'C' {
            score += LOST + PAPER;
        }
        // I chose scissors, opponent chose rock
        if round_move == 'Z' && opponent_move.get(i).unwrap() == 'A' {
            score += LOST + SCISSORS;
        }
        // I chose scissors, opponent chose paper
        if round_move == 'Z' && opponent_move.get(i).unwrap() == 'B' {
            score += WIN + SCISSORS;
        }
    });


    println!("Following the strategy guide, I get a score of: {score}");

    let mut score: u32 = 0;
    my_move.iter().enumerate().for_each(|(i, round_move)| {
        // Both players choose Rock
        if round_move == 'X' && opponent_move.get(i).unwrap() == 'A' {
            score += DRAW + ROCK;
        }
        // Both players chose Paper
        if round_move == 'Y' && opponent_move.get(i).unwrap() == 'B' {
            score += DRAW + PAPER;
        }
        // Both players chose scissors
        if round_move == 'Z' && opponent_move.get(i).unwrap() == 'C' {
            score += DRAW + SCISSORS;
        }
        // I chose rock, oppenent chose paper
        if round_move == 'X' && opponent_move.get(i).unwrap() == 'B' {
            score += LOST + ROCK;
        }
        // I chose rock, opponent chose scissors
        if round_move == 'X' && opponent_move.get(i).unwrap() == 'C' {
            score += WIN + ROCK;
        }
        // I chose paper, opponent chose rock
        if round_move == 'Y' && opponent_move.get(i).unwrap() == 'A' {
            score += WIN + PAPER;
        }
        // I chose paper, opponent chose scissors
        if round_move == 'Y' && opponent_move.get(i).unwrap() == 'C' {
            score += LOST + PAPER;
        }
        // I chose scissors, opponent chose rock
        if round_move == 'Z' && opponent_move.get(i).unwrap() == 'A' {
            score += LOST + SCISSORS;
        }
        // I chose scissors, opponent chose paper
        if round_move == 'Z' && opponent_move.get(i).unwrap() == 'B' {
            score += WIN + SCISSORS;
        }
    });
}
