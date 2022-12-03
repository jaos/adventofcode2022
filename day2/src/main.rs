/*
Rock, Paper, Scissors, Part 1

    Read input:
        column 1: opponent 
            Values: A=Rock, B=Paper, C=Scissors
        column 2: my?
            Values: X=Rock, Y=Paper, Z=Scissors
        Scoring:
            + shape score: 1=Rock, 2=Paper, 3=Scissors
            + win/draw/lose: 0=loss, 3=draw, 6=win

    Ex:
        A Y: (rock, 1), (paper, 2) == win(6) == 6+2
        B X: (paper, 2), (rock, 1) == loss(0) == 1+0
        C Z: (scissors, 3), (scissors, 3) == draw(3) == 3+3

Part 2

    column 2: X=lose, Y=draw, Z=win
    Pick appropriate response
*/
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum RoundResult {
    Lose = 0,
    Draw = 3,
    Win = 6,
}
impl RoundResult {
    fn from_str(v:&str) -> Self {
        match v {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("garbage"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum RockPaperScissor {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}
impl RockPaperScissor {
    fn from_opponent(v:&str) -> Self {
        match v {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissor,
            _ => panic!("garbage"),
        }
    }

    fn from_player(v:&str) -> Self {
        match v {
            "X" => Self::Rock,
            "Y" => Self::Paper,
            "Z" => Self::Scissor,
            _ => panic!("garbage"),
        }
    }

    fn for_outcome(&self, v:&str) -> Self {
        let outcome = RoundResult::from_str(v);
        match (*self, outcome) {
            (Self::Rock, RoundResult::Draw) => Self::Rock,
            (Self::Paper, RoundResult::Draw) => Self::Paper,
            (Self::Scissor, RoundResult::Draw) => Self::Scissor,

            (Self::Rock, RoundResult::Lose) => Self::Scissor,
            (Self::Paper, RoundResult::Lose) => Self::Rock,
            (Self::Scissor, RoundResult::Lose) => Self::Paper,

            (Self::Rock, RoundResult::Win) => Self::Paper,
            (Self::Paper, RoundResult::Win) => Self::Scissor,
            (Self::Scissor, RoundResult::Win) => Self::Rock,
        }
    }

    fn play(&self, other:&RockPaperScissor) -> (RoundResult, u8) {
        let result = match (*self, other) {
            (Self::Rock, Self::Scissor) => RoundResult::Win,
            (Self::Paper, Self::Rock) => RoundResult::Win,
            (Self::Scissor, Self::Paper) => RoundResult::Win,

            (Self::Rock, Self::Paper) => RoundResult::Lose,
            (Self::Paper, Self::Scissor) => RoundResult::Lose,
            (Self::Scissor, Self::Rock) => RoundResult::Lose,

            (Self::Rock, Self::Rock) => RoundResult::Draw,
            (Self::Paper, Self::Paper) => RoundResult::Draw,
            (Self::Scissor, Self::Scissor) => RoundResult::Draw,
        };
        (result, (*self as u8) + result as u8)
    }
}

fn rock_paper_scissors_part2(file_path:&String)
{
    let bf = BufReader::new(File::open(file_path).expect(file_path));

    let mut total_score:u32 = 0;
    for line in bf.lines() {
        let entry = line.unwrap();
        let entries: Vec<&str> = entry.split_whitespace().collect();
        if let [opponent, outcome] = entries[..] {
            let o = RockPaperScissor::from_opponent(opponent);
            let p = o.for_outcome(outcome);
            let (result, score) = p.play(&o);
            println!("{:>7} -vs- {:>7}: {:>4}, {} [{}]",
                format!("{:?}", o),
                format!("{:?}", p),
                format!("{:?}", result),
                score,
                entry
            );
            total_score += score as u32;
        }
    }
    println!("Total score is {}", total_score);
}

#[allow(dead_code)]
fn rock_paper_scissors_part1(file_path:&String)
{
    let bf = BufReader::new(File::open(file_path).expect(file_path));

    let mut total_score:u32 = 0;
    for line in bf.lines() {
        let entry = line.unwrap();
        let entries: Vec<&str> = entry.split_whitespace().collect();
        if let [opponent, player] = entries[..] {
            let o = RockPaperScissor::from_opponent(opponent);
            let p = RockPaperScissor::from_player(player);
            let (result, score) = p.play(&o);
            println!("{:>7} -vs- {:>7}: {:>4}, {} [{}]",
                format!("{:?}", o),
                format!("{:?}", p),
                format!("{:?}", result),
                score,
                entry
            );
            total_score += score as u32;
        }
    }
    println!("Total score is {}", total_score);
}

fn main()
{
    let args:Vec<String> = env::args().collect();
    if  args.len() == 2 {
        rock_paper_scissors_part2(&args[1]);
    } else {
        println!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }
}
