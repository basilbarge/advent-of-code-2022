fn main() {

    let playbook: Vec<&str> = include_str!("./input.txt").lines().collect();

    let mut entries: Vec<char> = Vec::new();

    for play in playbook {
        play.split_whitespace().for_each(|set| {
            entries.push(set.chars().last().unwrap());
        })
    }

    for entry in entries {
        println!("{entry}");
    }
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

