/* Cathode-Ray Tube

HandHeld CPU single register (X) starts up with a value of 1. Supports two isns
* addx [signed] ; cost 2 cycles
* noop ; cost 1 cycle

noop        [during cycle 1 X=1]
addx 3      [during cycle 2 X=1, after cycle 3, X=4]
addx -5     [during cycle 4 X=4, after cycle 5 X=-1]

Signal strenth: cycle number * X register value
*/
use std::env;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum Day10Error {
    OpParseError,
}
impl fmt::Display for Day10Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Op {
    NOOP,
    ADDX(isize),
}
impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl FromStr for Op {
    type Err = Day10Error;
    fn from_str(v:&str) -> Result<Self, Day10Error> {
        match v.split(" ").collect::<Vec<&str>>().as_slice() {
            // double word
            [o, v] => {
                match *o {
                    "addx" => {
                        match v.parse() {
                            Ok(opt) => Ok(Op::ADDX(opt)),
                            _ => Err(Day10Error::OpParseError),
                        }
                    },
                    _ => Err(Day10Error::OpParseError)
                }
            },
            [o] => {
                match *o {
                    "noop" => Ok(Op::NOOP),
                    _ => Err(Day10Error::OpParseError),
                }
            },
            _ => Err(Day10Error::OpParseError),
        }
    }
}

static CRT_BUF_LEN:usize = 40;
static CRT_BLANK_PIXEL:char = '░';
static CRT_ACTIVE_PIXEL:char = '▓';

#[derive(Debug)]
struct HandHeld {
    cycle_counter: isize,
    register_x: isize,
    signal_strenth_nvram: Vec<isize>,
    crt_buffer: Vec<char>,
}
impl HandHeld {
    fn cycle_accounting(&mut self) {
        self.cycle_counter += 1;
        self.signal_timer();
        self.crt_timer();
    }

    fn execute_op(&mut self, op:Op) {
        println!("{} performing operation {}", self, op);
        match op {
            Op::ADDX(v) => { // 2 cycles
                self.cycle_accounting();
                self.cycle_accounting();
                self.register_x += v;
            },
            Op::NOOP => { // 1 cycle
                self.cycle_accounting();
            },
        }
    }

    fn signal_timer(&mut self) {
        if self.cycle_counter == 20 || (self.cycle_counter - 20) % 40 == 0 {
            let signal_strength = self.cycle_counter * self.register_x;
            println!("{} signal strength for cycle {} is {}", self, self.cycle_counter, signal_strength);
            self.signal_strenth_nvram.push(signal_strength);
        }
    }
    fn crt_timer(&mut self) {
        let pixel = self.crt_buffer.len() as isize;
        let x = self.register_x;
        self.crt_buffer.push(if pixel == x || pixel == (x-1) || pixel == (x+1) {CRT_ACTIVE_PIXEL} else {CRT_BLANK_PIXEL});
        if self.cycle_counter % (CRT_BUF_LEN as isize) == 0 {
            println!("CRT: {}", self.crt_buffer.drain(..).collect::<String>());
        }
    }

}
impl Default for HandHeld {
    fn default() -> Self {
        Self{cycle_counter: 0, register_x: 1, signal_strenth_nvram: Vec::new(), crt_buffer: Vec::with_capacity(CRT_BUF_LEN)}
    }
}
impl fmt::Display for HandHeld {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[HandHeld({}: {}]", self.cycle_counter, self.register_x)
    }
}

fn day10(file_path:&str) -> Option<usize> {
    let mut handheld = HandHeld::default();

    let bf = BufReader::new(File::open(&file_path).expect(&file_path));
    for line in bf.lines() {
        match line {
            Ok(op_input) => {
                if let Ok(op) = op_input.parse::<Op>() {
                    handheld.execute_op(op);
                }
            },
            _ => {},
        }
    }
    Some(handheld.signal_strenth_nvram.iter().fold(0, |f,v| f+v) as usize)
}

fn main()
{
    let args:Vec<String> = env::args().collect();
    if let None = args.get(1) {
        println!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }
    dbg!(day10(&args[1]));
}

#[cfg(test)]
mod tests {
    use super::{Day10Error,HandHeld,Op};

    #[test]
    fn test_op() {
        assert_eq!("noop".parse::<Op>(), Ok(Op::NOOP));
        assert_eq!("addx 1".parse::<Op>(), Ok(Op::ADDX(1)));
        assert_eq!("addx -5".parse::<Op>(), Ok(Op::ADDX(-5)));
        assert_eq!("addx addx".parse::<Op>(), Err(Day10Error::OpParseError));
    }

    #[test]
    fn test_handheld() {
        let mut handheld = HandHeld::default();
        assert_eq!(handheld.cycle_counter, 0);
        assert_eq!(handheld.register_x, 1);
        assert_eq!(handheld.crt_buffer, vec![]);
        assert_eq!(handheld.signal_strenth_nvram, vec![]);

        handheld.execute_op(Op::NOOP);
        assert_eq!(handheld.register_x, 1);
        assert_eq!(handheld.cycle_counter, 1);
        assert_eq!(handheld.crt_buffer, vec!['#']);
        assert_eq!(handheld.signal_strenth_nvram, vec![]);

        handheld.execute_op(Op::ADDX(6));
        assert_eq!(handheld.register_x, 7);
        assert_eq!(handheld.cycle_counter, 3);
        assert_eq!(handheld.crt_buffer, vec!['#', '#', '#']);
        assert_eq!(handheld.signal_strenth_nvram, vec![]);

        handheld.execute_op(Op::ADDX(10));
        assert_eq!(handheld.register_x, 17);
        assert_eq!(handheld.cycle_counter, 5);
        assert_eq!(handheld.crt_buffer, vec!['#', '#', '#', '.', '.']);
        assert_eq!(handheld.signal_strenth_nvram, vec![]);
    }
}
