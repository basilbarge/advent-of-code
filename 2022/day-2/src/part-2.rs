fn main() {

    let playbook: Vec<&str> = include_str!("./input.txt").lines().collect();

    let mut entries: Vec<char> = Vec::new();
    let mut opponent_moves: Vec<Move> = Vec::new();
    let mut outcomes: Vec<Outcome> = Vec::new();

    for play in playbook {
        let first = play.split_whitespace().nth(0).unwrap().chars().last().unwrap();
        let second = play.split_whitespace().last().unwrap().chars().last().unwrap();
        let count = play.split_whitespace().count();

        match  first {
            'A' => opponent_moves.push(Move::Rock),
            'B' => opponent_moves.push(Move::Paper),
            'C' => opponent_moves.push(Move::Scissors),
            _ => panic!("Move type not supported")
        }

        match second {
            'X' => outcomes.push(Outcome::Lose),
            'Y' => outcomes.push(Outcome::Draw),
            'Z' => outcomes.push(Outcome::Win),
            _ => panic!("Outcome type not supported")
        }
    }

    let mut sum = 0;
    for (i, outcome) in outcomes.iter().enumerate() {
        let my_move = Move::determine_my_move((opponent_moves.get(i).unwrap(), outcome));

        sum += outcome.calculate_score(&my_move);
    }


    println!("My total score is {sum}");

}

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors
}

impl Move {
    fn determine_my_move(round: (&Move, &Outcome)) -> Move {
       match round {
            (Move::Rock, Outcome::Win) => Move::Paper,
            (Move::Rock, Outcome::Draw) => Move::Rock,
            (Move::Rock, Outcome::Lose) => Move::Scissors,
            (Move::Paper, Outcome::Win) => Move::Scissors,
            (Move::Paper, Outcome::Draw) => Move::Paper,
            (Move::Paper, Outcome::Lose) => Move::Rock,
            (Move::Scissors, Outcome::Win) => Move::Rock,
            (Move::Scissors, Outcome::Draw) => Move::Scissors,
            (Move::Scissors, Outcome::Lose) => Move::Paper,
            _ => Move::Rock

       }
    }
}

#[derive(Debug)]
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

