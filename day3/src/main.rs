/*
Each rucksack has 2 compartments. All items of a type go into exactly one of the two compartments.

Every item time is identified by a single letter (case sensitive).:w

Item list, on rucksack per line. First half of the line goes into on compartment, latter half in the other.
    vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw

Ex:
vJrwpWtwJgWrhcsFMMfFFhFp = (vJrwpWtwJgWr), (hcsFMMfFFhFp) [p is in both]
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL = (jqHRNqRjqzjGDLGL), (rsFMfFZSrLrFZsSL) [L is in both]
PmmdzqPrVvPwwTWBwg = (PmmdzqPrV), (vPwwTWBwg) [P is in both]
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn = (wMqvLMZHhHMvwLH), (jbvcjnnSBnvTQFn) [v is in both]
ttgJtRGJQctTZtZT = (ttgJtRGJ), (QctTZtZT) [t is in both]

CrZsJsPPZsGzwwsLwLmpwMDw = (CrZsJsPPZsGz), (wwsLwLmpwMDw) [s is in both]

Each letter is weighted by a->z = 1->26, A->Z = 27->52
Find the sum of the weighted duplicate value for all lines.

Part 2:
Every set of 3 lines is a group of 3 elves. The badge item type is the only letter found in all three.
vJrwpWtwJgWrhcsFMMfFFhFp [r]
  -        -
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL [r]
                -       - -
PmmdzqPrVvPwwTWBwg [r]
       -
*/
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[allow(dead_code)]
fn day3_part1(file_path:&String)
{
    let bf = BufReader::new(File::open(file_path).expect(file_path));
    let mut priority_sum:u32 = 0;
    for line in bf.lines() {
        let entry = line.unwrap();
        let (first, second) = entry.split_at(entry.len()/2);
        let firstset:HashSet<char> = first.chars().collect();
        let secondset:HashSet<char> = second.chars().collect();
        let intersection:u32 = match firstset.intersection(&secondset).take(1).next().expect("At least one duplicate") {
            v @ 'A'..='Z' => *v as u32 - 38,
            v @ 'a'..='z' => *v as u32 - 96,
            v => panic!("do not know how to map {}", v)
        };
        // println!("({}), ({}) [{}]", first, second, intersection);
        priority_sum += intersection;
    }
    println!("Priority sum is {}", priority_sum);
}

fn day3_part2(file_path:&String)
{
    let bf = BufReader::new(File::open(file_path).expect(file_path));
    let mut priority_sum:u32 = 0;

    let mut group:Vec<HashSet<char>> = Vec::with_capacity(3);
    for line in bf.lines() {
        let entry = line.unwrap();
        group.push(entry.chars().collect());

        if group.len() == 3 {
            let overlap = group[0].iter().filter(|i|
                group[1].contains(i) && group[2].contains(i)
            ).take(1).next().expect("At least one overlap");

            let overlap_value = match overlap {
                'A'..='Z' => *overlap as u32 - 38,
                'a'..='z' => *overlap as u32 - 96,
                _ => panic!("do not know how to map {}", overlap)
            };
            // println!("{}, {}", overlap, overlap_value);
            priority_sum += overlap_value;

            group.clear();
        }
    }
    println!("Priority sum is {}", priority_sum);
}

fn main()
{
    let args:Vec<String> = env::args().collect();
    if args.len() == 2 {
        day3_part2(&args[1]);
    } else {
        println!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }
}
