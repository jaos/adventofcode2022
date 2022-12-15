/*
 Regolith Reservoir
*/
use std::env;
use std::str::FromStr;
use std::fs::read_to_string;
use std::io::Write;

#[derive(Debug)]
enum Day14Error {
    PointParseError,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn parse_line(line:&str) -> Vec<Self> {
        line.split_ascii_whitespace()
            .filter(|p| *p != "->")
            .map(|i| i.parse::<Point>())
            .filter_map(|i| i.ok())
            .collect::<Vec<Point>>()
    }

    fn complete_line(&self, other:&Point) -> Vec<Point> {
        if self == other {
            vec![]
        } else {
            let mut points:Vec<Point> = Vec::new();
            for x in self.x.min(other.x)..=self.x.max(other.x) {
                for y in self.y.min(other.y)..=self.y.max(other.y) {
                    points.push(Point{x, y});
                }
            }
            points
        }
    }

    fn left(&mut self) {self.x -= 1; }
    fn right(&mut self) {self.x += 1; }
    fn down(&mut self) {self.y += 1; }
}

impl FromStr for Point {
    type Err = Day14Error;
    fn from_str(v:&str) -> Result<Self, Self::Err> {
        match v.split_once(",") {
            Some((xs,ys)) => match (xs.parse::<usize>(), ys.parse::<usize>()) {
                (Ok(x), Ok(y)) => Ok(Self{x, y}),
                _ => Err(Day14Error::PointParseError),
            },
            None => Err(Day14Error::PointParseError),
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point({},{})", self.x, self.y)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Map {
    data: Vec<char>,
    drop: Point,
    drop_counter: usize,
    lost_drops: bool,
    floor: usize,
}

impl Map {
    fn new() -> Self {
        let mut new = Self{data: vec!['░'; 800*300], drop: Point{x:500,y:0}, drop_counter: 0, lost_drops:false, floor:0};
        new.data[500] = '+';
        new
    }

    fn point_to_offset(&self, point:&Point) -> usize {
        point.y * 600 + point.x
    }

    fn can_move_to(&self, point:&Point) -> bool {
        let drop_offset = self.point_to_offset(&point);
        self.data[drop_offset] != '█' && self.data[drop_offset] != 'o'
    }

    fn fill(&mut self, points:&Vec<Point>) {
        let mut last_point:&Point = &points[0];
        for point in points {
            if point.y > self.floor {
                self.floor = point.y;
            }
            let offset = self.point_to_offset(point);
            self.data[offset] = '█';
            if point != last_point {
                for intermediate_point in last_point.complete_line(point) {
                    let offset = self.point_to_offset(&intermediate_point);
                    self.data[offset] = '█';
                    if intermediate_point.y > self.floor {
                        self.floor = intermediate_point.y;
                    }
                }
                last_point = point;
            }
        }
    }

    fn mark_drop(&mut self) {
        let drop_offset = self.point_to_offset(&self.drop);
        self.data[drop_offset] = 'o';
        self.drop = Point{x:500, y:0};
        self.drop_counter += 1;
    }

    fn can_drop(&self) -> bool {
        let mut test = self.drop.clone();
        test.down();
        self.can_move_to(&test)
    }
    fn can_move_left(&self) -> bool {
        let mut test = self.drop.clone();
        test.left();
        test.down();
        self.can_move_to(&test)
    }
    fn can_move_right(&self) -> bool {
        let mut test = self.drop.clone();
        test.right();
        test.down();
        self.can_move_to(&test)
    }

    fn move_drop(&mut self) {
        // part 1
        /*
        if self.drop.y >= self.floor {
            self.lost_drops = true;
            return;
        }
        */
        if self.drop.y == self.floor {
            //self.lost_drops = true;
            self.mark_drop();
            return;
        }

        if self.can_drop() {
            self.drop.down();
        } else if self.can_move_left() {
            self.drop.left();
        } else if self.can_move_right() {
            self.drop.right();
        } else {
            // part 2
            if self.drop.eq(&Point{x:500,y:0}) {
                self.lost_drops = true;
                self.mark_drop();
                return;
            }
            self.mark_drop();
        }
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut data = self.data.clone();
        data[self.point_to_offset(&self.drop)] = 'O';

        // let line = format!("\x1B[2J\n{}", self.data.chunks(600).map(|c| c.iter().collect::<String>()).collect::<Vec<String>>().join("\n"));
        // for test
        // let line = format!("\x1B[2J\n{}", data.chunks(600).map(|c| c.iter().skip(488).take(30).collect::<String>()).take(14).collect::<Vec<String>>().join("\n"));
        // for input
        let line = format!("\x1B[2J\n{}", data.chunks(600).map(|c| c.iter().skip(487).collect::<String>()).take(250).collect::<Vec<String>>().join("\n"));
        write!(f, "{}", line)
    }
}

fn main()
{
    let args:Vec<String> = env::args().collect();
    if let None = args.get(1) {
        println!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }

    let mut map = Map::new();
    if let Ok(input) = read_to_string(&args[1]) {
        for line in input.lines() {
            map.fill(&Point::parse_line(line));
        }
    }
    // part two... infinite floor below the lowest
    map.floor += 1;

    while !map.lost_drops {
        map.move_drop();
        /* visualization */
        // std::io::stdout().write(format!("{}", map).as_bytes());
        // std::thread::sleep(std::time::Duration::from_nanos(70700));
        // std::thread::sleep(std::time::Duration::from_millis(5));
    }
    std::io::stdout().write(format!("{}", map).as_bytes());
    println!("\n{}", map.drop_counter);
}
