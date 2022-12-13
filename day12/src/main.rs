/*
Hill Climbing Algorithm

Height map where a is lowest and z is highest
Special markers include:
    * S start|current position, is equal to a in value
    * E end|location for best signal, is equal to z in value

Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi

Find path to E in the fewest steps possible. Each step must be cardinal
direction and at most ONE change in height.

*/
use std::collections::{HashMap,VecDeque};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::fmt;
use std::time::Instant;

#[derive(Debug)]
enum Day12Error {
    MapParseError,
}

#[derive(Debug, Copy, PartialEq, Eq, Hash)]
struct PathStep {
    x: usize,
    y: usize,
    height: usize,
}

impl PathStep {
    fn new(x:usize, y:usize, height:usize) -> Self { Self{x,y, height} }
    fn as_char(&self) -> char {
        match char::from_u32((self.height + 97) as u32) {
            Some(c) => c,
            None => '.',
        }
    }
}

impl Clone for PathStep {
    fn clone(&self) -> Self {
        Self{..*self}
    }
}

#[derive(Debug)]
struct Map {
    cols: usize,
    rows: usize,
    data: Vec<PathStep>,
    start: PathStep,
    end: PathStep,
}

impl Map {
    fn parse_file(file_path:&str) -> Result<Self, Day12Error> {
        let bf = BufReader::new(File::open(&file_path).expect(&file_path));

        let mut start:Option<PathStep> = None;
        let mut end:Option<PathStep> = None;

        let mut cols:usize = 0;
        let mut rows:usize = 0;
        let mut data:Vec<PathStep> = Vec::new();
        for line in bf.lines().filter_map(|l| l.ok()) {
            cols = line.len();
            for (idx, c) in line.chars().enumerate() {
                let height = match c {
                    'a'..='z' => (u32::from(c) - 97) as usize,
                    'S' => (u32::from('a') - 97) as usize,
                    'E' => (u32::from('z') - 97) as usize,
                    _ => 0,
                };
                let step = PathStep::new(idx, rows, height);
                if c == 'S' {
                    start = Some(step.clone());
                }
                if c == 'E' {
                    end = Some(step.clone());
                }
                data.push(step)
            }
            rows += 1;
        }
        match (start, end) {
            (Some(s), Some(e)) => Ok(Self{cols, rows, data, start: s, end: e}),
            _ => Err(Day12Error::MapParseError),
        }
    }

    // maybe inline?
    fn available_steps(&self, step:&PathStep) -> Vec<&PathStep> {
        let mut steps:Vec<&PathStep> = Vec::with_capacity(4);
        let idx = step.y*self.cols +  step.x;
        // north
        if step.y > 0 {
            steps.push(&self.data[idx - self.cols]);
        }
        // south
        if step.y < (self.rows - 1) {
            steps.push(&self.data[idx + self.cols]);
        }
        // east
        if step.x < (self.cols - 1) {
            steps.push(&self.data[idx + 1]);
        }
        // west
        if step.x > 0 {
            steps.push(&self.data[idx - 1]);
        }
        steps.into_iter().filter(|s| s.height < step.height || s.height - step.height <= 1).collect()
    }

    // part 1
    fn path_to_the_end<'a>(&'a self, start:&'a PathStep) -> Option<Vec<&'a PathStep>> {
        let mut steps_todo:VecDeque<&PathStep> = VecDeque::from([start]);
        let mut steps_taken:HashMap<&PathStep, &PathStep> = HashMap::from([(start, start)]);

        while !steps_todo.is_empty() {
            if let Some(todo) = steps_todo.pop_front() {
                if *todo == self.end {
                    println!("Found the end at {:?}", todo);
                    break;
                }
                for step in self.available_steps(todo) {
                    if let None = steps_taken.get(step) {
                        steps_todo.push_back(step);
                        steps_taken.insert(step, todo);
                    }
                }
            }
        }

        let mut path:Vec<&PathStep> = Vec::new();
        let mut current:&PathStep = &self.end;
        while current != start {
            path.push(current);
            match steps_taken.get(current) {
                Some(step) => current = step,
                None => {
                    println!("shit! failed to walk backwards");
                    return None // egads?
                }
            }
        }
        Some(path)
    }

    // part 2
    fn path_to_lowest_elevation<'a>(&'a self) -> Option<Vec<&'a PathStep>> {
        self.data.iter()
            .filter(|step| step.height == 0)
            .filter_map(|step| self.path_to_the_end(step))
            .min_by_key(|step| step.len())
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lines:Vec<String> = self.data.chunks(self.cols).map(|line| {
            line.iter().map(|step| {
                if *step == self.start {
                    'S'
                } else if *step == self.end {
                    'E'
                } else { step.as_char()
                }
            }).collect::<String>()
        }).collect();
        let display = lines.join("\n");
        write!(f, "{}", display)
    }
}

fn day12(file_path:&str) -> Option<(usize, usize)>
{
    let mut part1 = 0;
    let mut part2 = 0;

    let now = Instant::now();
    if let Ok(map) = Map::parse_file(file_path) {
        println!("Map\n{}", map);
        if let Some(path) = map.path_to_the_end(&map.start) {
            part1 = path.len();
            //println!("Steps:");
            //for step in &path {
                //println!("\t {:?} {}", step, step.as_char());
            //}
            println!("Path from start to end in steps {}", part1);
        }
        if let Some(path) = map.path_to_lowest_elevation() {
            part2 = path.len();
            println!("Steps:");
            for step in &path {
                println!("\t {:?} {}", step, step.as_char());
            }
            println!("Path from to end lowest elevation in steps {}", part2);
        }
    }
    println!("{:.2?}", now.elapsed());
    Some((part1, part2))
}

fn main()
{
    let args:Vec<String> = env::args().collect();
    if let None = args.get(1) {
        println!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }
    dbg!(day12(&args[1]));
}
