use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

fn day1(file_path:&String) {
    let bf = BufReader::new(File::open(file_path).expect(&format!("Failed to open file: {}", file_path)));
    let mut elves:Vec<i32> = Vec::new();
    let mut counter:i32 = 0;
    for line in bf.lines() {
        if let Ok(s) = line {
            if s.is_empty() {
                elves.push(counter);
                counter = 0;
            } else {
                counter += s.parse().unwrap_or(0);
            }
        }
    }
    // reverse sort
    elves.sort_by(|e1, e2| e2.cmp(&e1));

    //part 1
    if let Some(e) = elves.first() {
        println!("Max elf has {} calories", e);
    }

    // part 2
    if elves.len() > 2 {
        let top_three = &elves[0..3];
        let top_three_total_calories:i32 = top_three.iter().sum();
        println!("Top three total calories: {}", top_three_total_calories);
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct ElfCaloryCounter {
    item_count: i32,
    total_calories: i32,
}

impl ElfCaloryCounter {
    pub fn new(item_count: i32, total_calories: i32) -> Self {
        ElfCaloryCounter {
            item_count,
            total_calories,
        }
    }
}

impl std::default::Default for ElfCaloryCounter {
    fn default() -> Self {
        ElfCaloryCounter::new(0, 0)
    }
}

#[allow(dead_code)]
fn struct_vec_p2(file_path:&String) {
    let mut elves:Vec<ElfCaloryCounter> = Vec::new();
    elves.push(ElfCaloryCounter::default());

    let bf = BufReader::new(File::open(file_path).expect(&format!("Failed to open file: {}", file_path)));
    for line in bf.lines() {
        let entry = line.unwrap();

        if entry.is_empty() {
            elves.push(ElfCaloryCounter::default());
        } else {
            let current_elf = elves.last_mut().unwrap();
            current_elf.item_count += 1;
            current_elf.total_calories += entry.parse().unwrap_or(0);
        }
    }

    elves.sort_by(|e1, e2| e2.total_calories.cmp(&e1.total_calories)); // reverse sort
    let max = elves.first().unwrap();
    println!("Max elf has {} calories with {} items", max.total_calories, max.item_count);

    let top_three = &elves[0..3];
    dbg!(top_three);
    let top_three_total_calories:i32 = top_three.iter().map(|e| e.total_calories).sum();
    println!("Top three total calories: {}", top_three_total_calories);
}

#[allow(dead_code)]
fn struct_vec_p1(file_path:&String) {
    let mut elves:Vec<ElfCaloryCounter> = Vec::new();
    elves.push(ElfCaloryCounter::default());

    let bf = BufReader::new(File::open(file_path).expect(&format!("Failed to open file: {}", file_path)));
    for line in bf.lines() {
        let entry = line.unwrap();

        if entry.is_empty() {
            elves.push(ElfCaloryCounter::default());
        } else {
            let current_elf = elves.last_mut().unwrap();
            current_elf.item_count += 1;
            current_elf.total_calories += entry.parse().unwrap_or(0);
        }

    }

    let max = elves.into_iter().max_by(|e1, e2| e1.total_calories.cmp(&e2.total_calories)).unwrap();
    println!("Max elf has {} calories with {} items", max.total_calories, max.item_count);
}

#[allow(dead_code)]
fn first_try(file_path:&String) {
    let bf = BufReader::new(File::open(file_path).expect("Failed to open file: {file_path}"));

    let mut max_calories = 0;
    let mut calories_for_current_elf = 0;

    for line in bf.lines() {
        let entry = line.unwrap();

        if entry.is_empty() {
            if calories_for_current_elf > max_calories {
                max_calories = calories_for_current_elf;
            }
            calories_for_current_elf = 0;
        } else {
            calories_for_current_elf += entry.parse().unwrap_or(0);
        }
    }
    println!("Max calories for an elf: {}", max_calories);
}

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    } 

    day1(&args[1]); 
}
