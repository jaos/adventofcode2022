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
use std::env;
use std::fs::read_to_string;
use std::ops::RangeInclusive;

/// parse a line into a pair of inclusive ranges
///
/// ```
/// let result = parse_line("0-1,2-3");
/// assert_eq!(result.ok_or("failed to parse line")?, (0..=1, 2..=3));
/// ```
pub fn parse_line(line: &str) -> Option<(RangeInclusive<u32>, RangeInclusive<u32>)>
{
    match line.trim().split_once(",") {
        Some((r1, r2)) => Some((parse_range(r1)?, parse_range(r2)?)),
        None => None
    }
}

/// parse a string into an inclusive range
///
/// ```
/// let result = parse_range("0-2");
/// assert_eq!(result.ok_or("failed to parse range")?, 0..=2);
/// ```
pub fn parse_range(range: &str) -> Option<RangeInclusive<u32>>
{
    match range.trim().split_once("-") {
        Some((range_start, range_end)) => {
            let r = range_start.trim().parse().ok()? ..= range_end.trim().parse().ok()?;
            Some(r)
        },
        None => None,
    }
}

/// check if two inclusive ranges overlap at all
///
/// ```
/// let result = overlaps_any(0..=3, 1..=3);
/// assert!(result);
/// let result = overlaps_any(0..=2, 4..=6);
/// assert!(!result);
///
/// ```
pub fn overlaps_any(r1: &RangeInclusive<u32>, r2: &RangeInclusive<u32>) -> bool
{
    return r1.contains(&r2.start()) || r1.contains(&r2.end()) || r2.contains(&r1.start()) || r2.contains(&r1.end());
}

/// check if two inclusive ranges overlap completely (one is a sub or super set)
///
/// ```
/// let result = overlaps_all(2..=3, 0..=3);
/// assert!(result);
/// ```
pub fn overlaps_all(r1: &RangeInclusive<u32>, r2: &RangeInclusive<u32>) -> bool
{
    return (r1.contains(&r2.start()) && r1.contains(&r2.end())) || (r2.contains(&r1.start()) && r2.contains(&r1.end()));
}

fn doday4(file_path:&String) -> Option<(u32, u32)>
{
    let mut answer_part1 = 0;
    let mut answer_part2 = 0;
    let bf = read_to_string(file_path).ok()?;
    for line in bf.lines() {
        let (r1, r2) = parse_line(&line)?;
        if overlaps_all(&r1, &r2) {
            answer_part1 += 1;
        }
        if overlaps_any(&r1, &r2) {
            answer_part2 += 1;
        }
    }
    Some((answer_part1, answer_part2))
}

fn main()
{
    let args:Vec<String> = env::args().collect();
    if let None = args.get(1) {
        println!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }
    println!("{:?}", doday4(&args[1]));
}


#[cfg(test)]
mod tests {
    use super::{parse_line, parse_range, overlaps_all, overlaps_any};

    #[test]
    fn test_parse_line() -> Result<(), String> {
        assert_eq!(parse_line("1-2,3-4").ok_or("Failed to parse line")?, (1..=2, 3..=4));
        Ok(())
    }
    #[test]
    fn test_parse_range() -> Result<(), String> {
        assert_eq!(parse_range("4-8").ok_or("failed to parse range")?, 4..=8);
        Ok(())
    }
    #[test]
    fn test_overlaps_all() -> Result<(), String> {
        let r1 = 3 ..= 5;
        let r2 = 2 ..= 6;
        let r3 = 0 ..= 2;
        assert!(overlaps_all(&r1, &r2));
        assert!(!overlaps_all(&r3, &r2));
        Ok(())
    }
    #[test]
    fn test_overlaps_any() -> Result<(), String> {
        let r1 = 3 ..= 5;
        let r2 = 2 ..= 6;
        let r3 = 0 ..= 2;
        assert!(overlaps_any(&r1, &r2));
        assert!(overlaps_any(&r2, &r3));
        assert!(!overlaps_any(&r1, &r3));
        Ok(())
    }
}
