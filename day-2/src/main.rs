const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;


fn main() {

    let playbook: Vec<&str> = include_str!("./input.txt").lines().collect();

    let mut my_move: Vec<&str> = Vec::new();
    let mut opponent_move: Vec<&str> = Vec::new();

    for play in playbook {
        let round = play.split(" ").collect::<Vec<&str>>();

        my_move.push(round[0]);
        opponent_move.push(round[1]);
    }

    let score = 0;
    my_move.iter().enumerate().for_each(|(i, round_move)| {
        
    })

}
