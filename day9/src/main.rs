/* Rope Bridge */
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;
use std::fmt;


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
struct DirectionParseError;
impl FromStr for Direction {
    type Err = DirectionParseError;
    fn from_str(v:&str) -> Result<Self, DirectionParseError> {
        match v {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(DirectionParseError),
        }
    }
}
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32
}
impl Point {
    fn new(x:i32, y:i32) -> Self {
        Self{x, y}
    }
    fn move_position(&mut self, count:u32, direction:Direction) {
        match direction {
            Direction::Up => self.y += count as i32,
            Direction::Down => self.y -= count as i32,
            Direction::Left => self.x -= count as i32,
            Direction::Right => self.x += count as i32,
        }
    }
    fn follow(&mut self, other:&Point) {

        // skip equal
        if self == other {
            return;
        }

        // skip adjacent
        let x_delta = other.x - self.x;
        let y_delta = other.y - self.y;
        let distance = f64::sqrt( (x_delta.pow(2) + y_delta.pow(2)) as f64) as u32;
        if distance == 1 {
            return; // already adjacent
        }
        // check to see if we share the same x or y
        if x_delta == 0 || y_delta == 0 {
            let d = match (x_delta, y_delta) {
                (x, y) if x == 0 && y < 0 => {Direction::Down},
                (x, y) if x == 0 && y > 0 => {Direction::Up},
                (x, y) if x < 0 && y == 0 => {Direction::Left},
                (x, y) if x > 0 && y == 0 => {Direction::Right},
                (_, _) => todo!("should never happen"),
            };
            self.move_position(1, d);
            return;
        }

        match (x_delta, y_delta) {
            (x, y) if x > 0 && y > 0 => {
                self.move_position(1, Direction::Right);
                self.move_position(1, Direction::Up);
            },
            (x, y) if x > 0 && y < 0 => {
                self.move_position(1, Direction::Right);
                self.move_position(1, Direction::Down);
            },
            (x, y) if x < 0 && y > 0 => {
                self.move_position(1, Direction::Left);
                self.move_position(1, Direction::Up);
            },
            (x, y) if x < 0 && y < 0 => {
                self.move_position(1, Direction::Left);
                self.move_position(1, Direction::Down);
            },
            (_, _) => todo!("should never happen"),
        }
    }
}
impl Clone for Point {
    fn clone(&self) -> Self {
        Self{..*self}
    }
}
impl Default for Point {
    fn default() -> Self {
        Self::new(0, 0)
    }
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

fn day7(file_path:&str) -> Option<usize>
{
    let tail_count = 10; // part1 == 2
    let mut knots = vec![Point::default(); tail_count];
    let mut visited_by_tail:HashSet<Point> = HashSet::new();

    let bf = BufReader::new(File::open(&file_path).expect(&file_path));
    for line in bf.lines() {
        let move_cmd = line.ok()?;
        match move_cmd.split_once(" ") {
            Some((d, c)) => {
                match (Direction::from_str(d), u32::from_str_radix(c, 10)) {
                    (Ok(d), Ok(c)) => {
                        for _ in 0..c {
                            let mut knots_iter = knots.iter_mut();
                            let mut last_knot = knots_iter.next()?;
                            // move the head first
                            last_knot.move_position(1, d);
                            // followers here
                            while let Some(knot)  = knots_iter.next() {
                                knot.follow(&last_knot);
                                last_knot = knot;
                            }
                            visited_by_tail.insert(last_knot.clone());
                        }
                    },
                    (_, _) => return None,
                }
            },
            _ => {return None;}
        }
    }

    Some(visited_by_tail.len())
}

fn main()
{
    let args:Vec<String> = env::args().collect();
    if let None = args.get(1) {
        println!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }
    dbg!(day7(&args[1]));
}
