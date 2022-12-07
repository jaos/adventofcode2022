/* Tuning Trouble

Series of characters as a signal from the device
look for start of packet marker, 4 characters that are all different

mjqjpqmgbljsphdztnvjfqwrcgsmlb


bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 5
nppdvjthqldpwncqszvftbrmjlhg: first marker after character 6
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 10
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 11

*/
use std::env;
use std::collections::HashSet;
use std::fs::read_to_string;

fn day6(input_line:&str, msg_len:usize) -> Option<usize>
{
    let input_line_len = input_line.len();

    let mut window:HashSet<char> = HashSet::with_capacity(4);
    for offset in 0..input_line_len {
        if offset + msg_len < input_line_len {
            window.extend(input_line[offset..offset+msg_len].chars());
            if window.len() == msg_len {
                return Some(offset+msg_len);
            }
            window.clear();
        }
    }
    None
}

fn main()
{
    let args:Vec<String> = env::args().collect();
    if let None = args.get(1) {
        println!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }
    if let Ok(line) = read_to_string(&args[1]) {
        if let Some(first_marker) = day6(&line, 4) {
            println!("first marker is {} for part1", first_marker);
        }
        if let Some(first_marker) = day6(&line, 14) {
            println!("first marker is {} for part2", first_marker);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day6;

    #[test]
    fn test_day6() {
        assert_eq!(day6("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), Some(5));
        assert_eq!(day6("nppdvjthqldpwncqszvftbrmjlhg", 4), Some(6));
        assert_eq!(day6("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), Some(10));
        assert_eq!(day6("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), Some(11));

        assert_eq!(day6("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), Some(19));
        assert_eq!(day6("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), Some(23));
        assert_eq!(day6("nppdvjthqldpwncqszvftbrmjlhg", 14), Some(23));
        assert_eq!(day6("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), Some(29));
        assert_eq!(day6("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), Some(26));

    }
}
