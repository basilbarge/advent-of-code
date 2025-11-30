struct Round {
    player_move: Move,
    opponent_move: Move,
}

impl Round {
    fn determine_outcome(&self) -> Outcome {
        match &self.player_move {
            Move::Rock => match &self.opponent_move {
                Move::Rock => Outcome::Draw,
                Move::Paper => Outcome::Lose,
                Move::Scissors => Outcome::Win

            },
            Move::Paper =>  match &self.opponent_move {
                Move::Rock => Outcome::Win,
                Move::Paper => Outcome::Draw,
                Move::Scissors => Outcome::Lose

            },
            Move::Scissors => match &self.opponent_move {
                Move::Rock => Outcome::Lose,
                Move::Paper => Outcome::Win,
                Move::Scissors => Outcome::Draw
            }

        }
    }

    fn calculate_score(&self, result: Outcome) -> u32 {
        let mut sum = 0;
        match &self.player_move {
            Move::Rock => sum += 1,
            Move::Paper => sum += 2,
            Move::Scissors => sum += 3,
        }

        match result {
            Outcome::Lose => sum += 0,
            Outcome::Draw => sum += 3,
            Outcome::Win => sum += 6
        }

        sum
    }
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

    let mut rounds: Vec<Round> = Vec::new();

    for play in playbook {
        let round: Vec<&str> = play.split(" ").collect();
        let mut opponent_move: Option<Move> = None;
        let mut player_move: Option<Move> = None;

        if round[0].chars().as_str() == "A" {
            opponent_move = Some(Move::Rock);
        } else if round[0].chars().as_str() == "B" {
            opponent_move = Some(Move::Paper);
        } else if round[0].chars().as_str() == "C" {
            opponent_move = Some(Move::Scissors);
        }

        if round[1].chars().as_str() == "X" {
            player_move = Some(Move::Rock);
        } else if round[1].chars().as_str() == "Y" {
            player_move = Some(Move::Paper);
        } else if round[1].chars().as_str() == "Z" {
            player_move = Some(Move::Scissors);
        }

        let r = Round {
            player_move: match player_move {
                Some(game_move) => game_move,
                None => panic!("Move not determined for player")
            },
            opponent_move: match opponent_move {
                Some(game_move) => game_move,
                None => panic!("Move not determined for opponent")
            }
        };

        rounds.push(r);
    }

    let score: u32 = rounds.iter().map(|r| r.calculate_score(r.determine_outcome())).sum();

    println!("Following the strategy guide, I get a score of: {score}");
}
