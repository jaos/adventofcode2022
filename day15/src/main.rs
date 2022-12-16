/* Beacon Exclusion Zone
*/
use std::env;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum Day15Error {
    PointParseError,
    SensorParseError,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x:isize, y:isize) -> Self {
        Self{x, y}
    }
    fn manhatten_distance(&self, other:&Point) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }

    fn tuning_frequency(&self) -> usize {
        (self.x * 4000000 + self.y) as usize
    }
}

impl FromStr for Point {
    type Err = Day15Error;
    fn from_str(v:&str) -> Result<Self, Self::Err> {
        match v.split_once(", ") {
            Some((p1, p2)) => {
                match (p1.split_once("="), p2.split_once("=")) {
                    (Some(("x", xs)), Some(("y", ys))) => {
                        match (xs.parse::<isize>(), ys.parse::<isize>()) {
                            (Ok(x), Ok(y)) => Ok(Point::new(x, y)),
                            _ => Err(Day15Error::PointParseError),
                        }
                    },
                    _ => Err(Day15Error::PointParseError),
                }
            },
            _ => Err(Day15Error::PointParseError),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Sensor {
    location: Point,
    beacon: Point,
}

impl Sensor {
    fn new(location:Point, beacon:Point) -> Self {
        Self{location, beacon}
    }
}

impl FromStr for Sensor {
    type Err = Day15Error;
    fn from_str(v:&str) -> Result<Self, Day15Error> {
        match v.replace("Sensor at ", "").replace(" closest beacon is at ", "").split_once(":") {
            Some((location_s, beacon_s)) => {
                match (location_s.parse::<Point>(), beacon_s.parse::<Point>()) {
                    (Ok(l), Ok(b)) => Ok(Self::new(l, b)),
                    _ => Err(Day15Error::SensorParseError),
                }
            },
            _ => Err(Day15Error::SensorParseError),
        }
    }
}

type Range = Vec<(isize, isize)>;

struct SensorMap {
    sensors: Vec<Sensor>,
}
impl SensorMap {
    fn new(sensors:Vec<Sensor>) ->  Self {
        Self{sensors}
    }

    fn point_ranges_covered_for_y_axis(&self, y:isize) -> Range {
        let mut sensor_location_ranges:Vec<(Point, Point)> = Vec::new();
        // let mut beacon_on_y_axis_count = 0;

        for sensor in &self.sensors {
            // for each location, we need the manhatten distance to it's beacon... then we can get the distance
            // to our y axis and then figure out the range for that sensor.
            //          S
            //     b----|  <6 distance>
            //          |
            //y=     ---|--- <3 to y, so -3 to +3 of the sensor's x>
            let sensor_location_on_y = Point::new(sensor.location.x, y);
            let sensor_location_to_y_distance = sensor.location.manhatten_distance(&sensor_location_on_y) as isize;
            let sensor_location_to_beacon_distance = sensor.location.manhatten_distance(&sensor.beacon) as isize;
            if sensor_location_to_y_distance <= sensor_location_to_beacon_distance {
                let x_delta = (sensor_location_to_beacon_distance - sensor_location_to_y_distance).abs();
                let start_x = sensor_location_on_y.x - x_delta;
                let end_x = sensor_location_on_y.x + x_delta;
                sensor_location_ranges.push((
                    Point::new(start_x, y),
                    Point::new(end_x, y)
                ));
            }

            // if sensor.beacon.y == y {
                // beacon_on_y_axis_count += 1;
            // }
        }

        if sensor_location_ranges.is_empty() {
            return vec![];
        }

        // merge ranges
        sensor_location_ranges.sort();
        let mut merged_location_ranges:Range = Vec::new();
        merged_location_ranges.push((sensor_location_ranges[0].0.x, sensor_location_ranges[0].1.x));

        for idx in 1..sensor_location_ranges.len() {
            let start_x = sensor_location_ranges[idx].0.x;
            let end_x = sensor_location_ranges[idx].1.x;

            let idx_of_merged = merged_location_ranges.len()-1;
            let last = &mut merged_location_ranges[idx_of_merged];
            if (last.0..=last.1).contains(&start_x) || (last.0..=last.1).contains(&end_x) || last.1+1 == start_x {
                *last = (last.0.min(start_x), last.1.max(end_x));
            } else {
                merged_location_ranges.push((start_x, end_x));
            }
        }

        merged_location_ranges
    }

    fn part1(&self, y:isize) -> usize {
        let mut points_covered = 0;
        for r in self.point_ranges_covered_for_y_axis(y) {
            points_covered += r.1 - r.0;
        }
        // points_covered -= beacon_on_y_axis_count; // TODO do we count these beacon locations?
        points_covered as usize
    }

    fn part2(&self, max_value:usize) -> usize {
        let mut available_point:Option<Point> = None;

        for y in 0..=max_value {

            let points_covered = self.point_ranges_covered_for_y_axis(y as isize);
            if points_covered.len() <= 1 {
                continue;
            }

            let last_range = &points_covered[0];
            for idx in 1..points_covered.len() {
                let range = &points_covered[idx];
                let delta = range.0 - last_range.1;
                assert!(delta >= 1);
                for d in 1..delta {
                    available_point = Some(Point::new(range.0-d, y as isize));
                    break; // only first one or do we only expect one?
                }
            }

        }

        if let Some(p) = available_point {
            p.tuning_frequency()
        } else {
            0
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

    if let Ok(data) = read_to_string(&args[1]) {
        let sensors = data.lines().filter_map(|l| l.parse::<Sensor>().ok()).collect::<Vec<Sensor>>();

        let map = SensorMap::new(sensors);
        // println!("Part1 test: {}", map.part1(10));
        // println!("Part2 test: {}", map.part1(20));

        println!("Part1: {}", map.part1(2000000));
        println!("Part2: {}", map.part2(4000000));
    }
}

#[cfg(test)]
mod tests {
    use super::{Day15Error, Point, Sensor, SensorMap};

    #[test]
    fn test_point() {
        assert_eq!("x=20, y=1".parse::<Point>(), Ok(Point::new(20,1)));
        assert_eq!("x=20y=1".parse::<Point>(), Err(Day15Error::PointParseError));
        assert_eq!(Point::new(8, 7).manhatten_distance(&Point::new(2,10)), 9);
        assert_eq!(Point::new(14, 11).tuning_frequency(), 56000011);
    }

    #[test]
    fn test_sensor() {
        assert_eq!("Sensor at x=20, y=14: closest beacon is at x=25, y=17".parse::<Sensor>(), Ok(Sensor::new(Point::new(20, 14), Point::new(25, 17))));
        assert_eq!("at x=20, y=14: beacon is at x=25, y=17".parse::<Sensor>(), Err(Day15Error::SensorParseError));
    }

    #[test]
    fn test_sensormap() {
        let sensors = vec![
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15".parse::<Sensor>().unwrap(),
            "Sensor at x=9, y=16: closest beacon is at x=10, y=16".parse::<Sensor>().unwrap(),
            "Sensor at x=13, y=2: closest beacon is at x=15, y=3".parse::<Sensor>().unwrap(),
            "Sensor at x=12, y=14: closest beacon is at x=10, y=16".parse::<Sensor>().unwrap(),
            "Sensor at x=10, y=20: closest beacon is at x=10, y=16".parse::<Sensor>().unwrap(),
            "Sensor at x=14, y=17: closest beacon is at x=10, y=16".parse::<Sensor>().unwrap(),
            "Sensor at x=8, y=7: closest beacon is at x=2, y=10".parse::<Sensor>().unwrap(),
            "Sensor at x=2, y=0: closest beacon is at x=2, y=10".parse::<Sensor>().unwrap(),
            "Sensor at x=0, y=11: closest beacon is at x=2, y=10".parse::<Sensor>().unwrap(),
            "Sensor at x=20, y=14: closest beacon is at x=25, y=17".parse::<Sensor>().unwrap(),
            "Sensor at x=17, y=20: closest beacon is at x=21, y=22".parse::<Sensor>().unwrap(),
            "Sensor at x=16, y=7: closest beacon is at x=15, y=3".parse::<Sensor>().unwrap(),
            "Sensor at x=14, y=3: closest beacon is at x=15, y=3".parse::<Sensor>().unwrap(),
            "Sensor at x=20, y=1: closest beacon is at x=15, y=3".parse::<Sensor>().unwrap(),
        ];
        let map = SensorMap::new(sensors);
        assert_eq!(map.point_ranges_covered_for_y_axis(10), vec![(-2, 24)]);
        assert_eq!(map.part1(10), 26);
        assert_eq!(map.part2(20), 56000011);
    }
}
