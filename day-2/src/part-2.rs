const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;
const LOST: u32 = 0;
const DRAW: u32 = 3;
const WIN: u32 = 6;

impl Round {

}

enum Move {
    Rock,
    Paper,
    Scissors
}

impl Move {
    fn determine_outcome(&self, other_move: &Move) -> Outcome {
        match &self {
            Move::Rock => match other_move {
                Move::Rock => Outcome::Draw,
                Move::Paper => Outcome::Lose,
                Move::Scissors => Outcome::Win
            },
            Move::Paper =>  match other_move {
                Move::Rock => Outcome::Win,
                Move::Paper => Outcome::Draw,
                Move::Scissors => Outcome::Lose
            },
            Move::Scissors => match other_move {
                Move::Rock => Outcome::Lose,
                Move::Paper => Outcome::Win,
                Move::Scissors => Outcome::Draw
            }
        }
    }
}

enum Outcome {
    Win,
    Lose,
    Draw
}

impl Outcome {
    fn calculate_score(&self, player_move: &Move) -> u32 {
        let mut sum = 0;
        match player_move {
            Move::Rock => sum += 1,
            Move::Paper => sum += 2,
            Move::Scissors => sum += 3,
        }

        match &self {
            Outcome::Lose => sum += 0,
            Outcome::Draw => sum += 3,
            Outcome::Win => sum += 6
        }

        sum
    }
}

fn main() {

    let playbook: Vec<&str> = include_str!("./input.txt").lines().collect();


    


}
