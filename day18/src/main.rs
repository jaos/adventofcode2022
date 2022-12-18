/*
Boiling Boulders
*/
use std::env;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Day18Error {
    Point3ParseError,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point3(isize, isize, isize);

impl Point3 {
    fn distance(&self, other:&Self) -> usize {
        f64::sqrt(((self.0-other.0).pow(2) + (self.1-other.1).pow(2) + (self.2-other.2).pow(2)) as f64) as usize
    }
    fn to_right(&self) -> Self { Self{0:self.0 + 1, 1:self.1,     2:self.2 } }
    fn to_left(&self)  -> Self { Self{0:self.0 - 1, 1:self.1,     2:self.2 } }
    fn to_up(&self)    -> Self { Self{0:self.0,     1:self.1 + 1, 2:self.2 } }
    fn to_down(&self)  -> Self { Self{0:self.0,     1:self.1 - 1, 2:self.2 } }
    fn to_front(&self) -> Self { Self{0:self.0,     1:self.1,     2:self.2 + 1 } }
    fn to_back(&self)  -> Self { Self{0:self.0,     1:self.1,     2:self.2 - 1 } }
}
impl FromStr for Point3 {
    type Err = Day18Error;
    fn from_str(v:&str) -> Result<Self, Self::Err> {
        match v.split(",").collect::<Vec<&str>>().as_slice() {
            [xs,ys,zs] => {
                match (xs.parse::<isize>(), ys.parse::<isize>(), zs.parse::<isize>()) {
                    (Ok(x), Ok(y), Ok(z)) => Ok(Point3(x,y,z)),
                    _ => Err(Day18Error::Point3ParseError),
                }
            },
            _ => Err(Day18Error::Point3ParseError),
        }
    }
}

fn main()
{
    let args:Vec<String> = env::args().collect();
    if let None = args.get(1) {
        println!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }

    let cube_of_air = Point3(2,2,5);

    if let Ok(data) = read_to_string(&args[1]) {
        let points = data.lines().map(|l| l.parse::<Point3>()).filter_map(|l| l.ok()).collect::<Vec<Point3>>();
        let mut point_adjacency:HashMap<&Point3, usize> = points.iter().map(|p| (p, 0)).collect::<HashMap<_, _>>();
        for p1 in &points {
            if p1.eq(&cube_of_air) {
                continue;
            }
            let mut exposed_surface_count = 0;
            let sides = vec![p1.to_right(), p1.to_left(), p1.to_up(), p1.to_down(), p1.to_front(), p1.to_back()];
            for side in &sides {
                if let None = point_adjacency.get(&side) {
                    if !side.eq(&cube_of_air) {
                        exposed_surface_count += 1;
                    }
                }
            }
            if let Some(adjacency_count) = point_adjacency.get_mut(p1) {
                *adjacency_count = exposed_surface_count;
            }
        }
        let part1:usize = point_adjacency.iter().map(|i| i.1).sum();
        println!("Part1: {}", part1);
    }
}

#[cfg(test)]
mod tests {
    use super::{Day18Error,Point3};

    #[test]
    fn test_point3() {
        assert_eq!("2,2,2".parse::<Point3>(), Ok(Point3(2,2,2)));
        assert_eq!("1,9,z".parse::<Point3>(), Err(Day18Error::Point3ParseError));
        assert_eq!("0".parse::<Point3>(), Err(Day18Error::Point3ParseError));
        assert_eq!(Point3(1,1,1).distance(&Point3(2,1,1)), 1);
        assert_eq!(Point3(2,2,2).distance(&Point3(1,2,2)), 1);
        assert_eq!(Point3(6,4,-3).distance(&Point3(2,-8,3)), 14);
    }
}
