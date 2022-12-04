fn main() {
    let file = std::env::args().nth(1).unwrap();
    let data = std::fs::read_to_string(file).unwrap();

    let parsed = parse(&data);
    let solved = solve(&parsed);
    println!("{solved}");
}

#[derive(Debug, PartialEq, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq, Clone)]
enum Outcome {
    //X
    Lose,
    //Y
    Draw,
    //Z
    Win,
}

impl Hand {
    fn shape_score(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn beats(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }

    fn loses(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }

    fn outcome_score(&self, opponent: &Hand) -> u32 {
        if &self.beats() == opponent {
            6
        } else if &opponent.beats() == self {
            0
        } else {
            3
        }
    }

    fn to_outcome(&self, outcome: &Outcome) -> Hand {
        if outcome == &Outcome::Draw {
            self.clone()
        } else if outcome == &Outcome::Win {
            self.loses()
        } else {
            self.beats()
        }
    }
}

#[test]
fn test_outcome_score() {
    assert_eq!(Hand::Paper.outcome_score(&Hand::Rock), 6);
    assert_eq!(Hand::Rock.outcome_score(&Hand::Paper), 0);
    assert_eq!(Hand::Scissors.outcome_score(&Hand::Scissors), 3);
}

fn parse_hand(input: char) -> Hand {
    match input {
        'A' => Hand::Rock,
        'B' => Hand::Paper,
        'C' => Hand::Scissors,
        _ => unreachable!(),
    }
}

fn parse_outcome(input: char) -> Outcome {
    match input {
        'X' => Outcome::Lose,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => unreachable!(),
    }
}

fn parse(data: &str) -> Vec<(Hand, Outcome)> {
    data.lines()
        .map(|line| {
            let left = parse_hand(line.chars().nth(0).unwrap());
            let right = parse_outcome(line.chars().nth(2).unwrap());
            (left, right)
        })
        .collect()
}

#[test]
fn test_parse_hand() {
    assert_eq!(parse_hand('A'), Hand::Rock);
    assert_eq!(parse_hand('B'), Hand::Paper);
    assert_eq!(parse_hand('C'), Hand::Scissors);
}

#[test]
fn test_parse() {
    let data = include_str!("../example.txt");
    let parsed = parse(data);
    assert_eq!(
        parsed,
        vec![
            (Hand::Rock, Outcome::Draw),
            (Hand::Paper, Outcome::Lose),
            (Hand::Scissors, Outcome::Win)
        ]
    );
}

fn solve(input: &Vec<(Hand, Outcome)>) -> u32 {
    input
        .iter()
        .map(|(opponent, outcome)| {
            let hand = opponent.to_outcome(outcome);
            hand.shape_score() + hand.outcome_score(opponent)
        })
        .sum()
}

#[test]
fn can_solve() {
    let data = include_str!("../example.txt");
    let parsed = parse(data);
    assert_eq!(solve(&parsed), 15);
}
