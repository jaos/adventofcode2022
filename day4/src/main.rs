/*
Elves pair up and make section assignments for cleaning. Detect overlap/duplication for the pairs.
Part 1: In how many assignment pairs does one range fully contain the other?
    2-4,6-8 (2,3,4), (6,7,8) [no overlap]
    2-3,4-5 (2,3), (4,5) [no overlap]
    5-7,7-9 (5,6,7), (7,8,9) [partial overlap]
    2-8,3-7 (2,3,4,5,6,7,8), (3,4,5,6,7) [complete overlap]
    6-6,4-6 (6), (4,5,6) [complete overlap]
    2-6,4-8 (2,3,4,5,6) (4,5,6,7,8) [partial overlap]
    Total completely overlapping: 2
Part 2:
    2-4,6-8 (2,3,4), (6,7,8) [no overlap]
    2-3,4-5 (2,3), (4,5) [no overlap]
    5-7,7-9 (5,6,7), (7,8,9) [partial overlap]
    2-8,3-7 (2,3,4,5,6,7,8), (3,4,5,6,7) [complete overlap]
    6-6,4-6 (6), (4,5,6) [complete overlap]
    2-6,4-8 (2,3,4,5,6) (4,5,6,7,8) [partial overlap]
    Total even partially overlapping: 4
*/
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
enum Day4Error {
    FileError(std::io::Error),
    MustBeTwoEntriesError,
    NumberParseError(std::num::ParseIntError),
}
impl From<std::num::ParseIntError> for Day4Error {
    fn from(error: std::num::ParseIntError) -> Self {
        Self::NumberParseError(error)
    }
}
impl From<std::io::Error> for Day4Error {
    fn from(error: std::io::Error) -> Self {
        Self::FileError(error)
    }
}

#[allow(dead_code)]
enum Part {
    PART1,
    PART2,
}

fn day4(file_path:&String, part:Part) -> Result<u32, Day4Error>
{
    let bf = BufReader::new(File::open(file_path).expect(file_path));
    let mut answer_part1 = 0;
    let mut answer_part2 = 0;
    for line in bf.lines() {
        let entry = line?; // io::Error

        let mut ranges:Vec<HashSet<u32>> = Vec::new();
        for range_str in entry.trim().split(",").take(2) {
            let r:Vec<&str> = range_str.trim().split("-").take(2).collect();
            if let [b, e] = r[..] {
                let start:u32 = b.trim().parse()?; // ParseIntError
                let end:u32 = e.trim().parse()?; // ParseIntError
                ranges.push((start..=end).collect());
            } else {
                return Err(Day4Error::MustBeTwoEntriesError);
            }
        }
        if ranges.len() != 2 {
            return Err(Day4Error::MustBeTwoEntriesError);
        }
        if ranges[0].is_subset(&ranges[1]) || ranges[0].is_superset(&ranges[1]) {
            answer_part1 += 1;
        }
        if let Some(_) = ranges[0].intersection(&ranges[1]).take(1).next() {
            answer_part2 += 1;
        }
    }

    match part {
        Part::PART1 => Ok(answer_part1),
        Part::PART2 => Ok(answer_part2),
    }
}

fn usage(argv0:&str)
{
    println!("Usage: {} <path>", argv0);
    std::process::exit(1);
}

fn main()
{
    let args:Vec<String> = env::args().collect();
    if let None = args.get(1) {
        usage(&args[0]);
    }
    match day4(&args[1], Part::PART2) {
        Ok(answer) => println!("Answer: {}", answer),
        Err(e) => { println!("Error {:?}", e); },
    }
}
