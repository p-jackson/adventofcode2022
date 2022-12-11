use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Copy, Clone)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

fn main() {
    let file_path = "input.txt";
    let f = File::open(file_path)
        .expect(format!("Should have been able to read {}", file_path).as_str());
    let reader = BufReader::new(f);

    let total_score = reader
        .lines()
        .filter_map(|line_result| line_result.map(|line| translate_line(&line)).ok())
        .map(|(m, outcome)| {
            get_outcome_score(outcome) + get_choice_score(get_correct_move(m, outcome))
        })
        .sum::<u32>();

    println!("{}", total_score);
}

fn translate_line(line: &str) -> (Move, Outcome) {
    let mut correct_move = Move::Rock;
    let mut outcome = Outcome::Lose;

    for c in line.chars() {
        if c == 'A' || c == 'B' || c == 'C' {
            correct_move = match c {
                'A' => Move::Rock,
                'B' => Move::Paper,
                'C' => Move::Scissors,
                _ => Move::Rock,
            };
        } else if c == 'X' || c == 'Y' || c == 'Z' {
            outcome = match c {
                'X' => Outcome::Lose,
                'Y' => Outcome::Draw,
                'Z' => Outcome::Win,
                _ => Outcome::Lose,
            };
        }
    }

    (correct_move, outcome)
}

fn get_correct_move(m: Move, outcome: Outcome) -> Move {
    match outcome {
        Outcome::Lose => get_losing_move(m),
        Outcome::Draw => m,
        Outcome::Win => get_winning_move(m),
    }
}

fn get_winning_move(m: Move) -> Move {
    match m {
        Move::Rock => Move::Paper,
        Move::Paper => Move::Scissors,
        Move::Scissors => Move::Rock,
    }
}

fn get_losing_move(m: Move) -> Move {
    match m {
        Move::Rock => Move::Scissors,
        Move::Paper => Move::Rock,
        Move::Scissors => Move::Paper,
    }
}

fn get_choice_score(m: Move) -> u32 {
    match m {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

fn get_outcome_score(outcome: Outcome) -> u32 {
    match outcome {
        Outcome::Lose => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    }
}
