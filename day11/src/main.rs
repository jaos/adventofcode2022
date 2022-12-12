/* Monkey in the Middle

Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1

*/
use std::env;
use std::fmt;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug)]
enum Day11Error {
    MonkeyParseError,
}

#[derive(Debug, PartialEq, Eq)]
struct Item {
    worry_level: usize,
}
impl Item {
    fn new(worry_level:usize) -> Self {
        Self {worry_level}
    }
}
impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.worry_level)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Throw {
    monkey: usize,
    item: Item,
}
impl Throw {
    fn new(monkey:usize, item:Item) -> Self {
        Self{monkey, item}
    }
}

struct Monkey {
    id: usize,
    items: Vec<Item>,
    worry_level_mod: Box<dyn Fn(usize) -> usize>,
    throws_to: (usize, usize),
    divisible_by: usize,
    inspection_count: usize,
}

impl Monkey {
    fn throws(&mut self, part2:Option<usize>) -> Vec<Throw> {
        self.inspection_count += self.items.len();
        let throws = self.throws_to;
        let worry_modifier = &self.worry_level_mod;
        let test = self.divisible_by;
        self.items.drain(..).map(|mut item| {
            // inspect item... modify worry level (part1: divide by 3, part 2 modulus)
            item.worry_level = match part2 {
                Some(v) => (*worry_modifier)(item.worry_level) % v,
                None => (*worry_modifier)(item.worry_level) / 3,
            };
            let destination_monkey = if item.worry_level % test == 0 { throws.0 } else { throws.1 };
            Throw::new(destination_monkey, item)
        }).collect::<Vec<Throw>>()
    }
    fn catch(&mut self, throw:Throw) {
        assert_eq!(self.id, throw.monkey);
        self.items.push(throw.item);
    }
}

impl FromStr for Monkey {
    type Err = Day11Error;

    // egads this is ugly
    fn from_str(def:&str) -> Result<Self, Day11Error> {
        let lines = def.split("\n").collect::<Vec<&str>>();
        assert!(lines.len() >= 6);
        let id = lines[0].replace("Monkey ", "").replace(":", "")
            .parse::<usize>().or(Err(Day11Error::MonkeyParseError))?;
        let items:Vec<Item> = lines[1].replace("  Starting items: ", "").replace(" ", "").split(",")
            .map(|i| i.parse::<usize>().or(Err(Day11Error::MonkeyParseError)))
            .filter_map(|i| i.ok()).map(|w| Item::new(w)).collect::<Vec<Item>>();

        let w = lines[2].replace("  Operation: new = ", "");
        let worry_modification_parts:Vec<&str> = w.split(" ").collect();
        let divisible_by = lines[3].replace("  Test: divisible by ", "")
            .parse::<usize>().or(Err(Day11Error::MonkeyParseError))?;
        let true_throw = lines[4].replace("    If true: throw to monkey ", "")
            .parse::<usize>().or(Err(Day11Error::MonkeyParseError))?;
        let false_throw = lines[5].replace("    If false: throw to monkey ", "")
            .parse::<usize>().or(Err(Day11Error::MonkeyParseError))?;

        Ok(Monkey{
            id,
            items,
            worry_level_mod: match worry_modification_parts[..] {
                ["old", "+", "old"] => {
                    Box::new(|worry_level| {
                        worry_level.wrapping_add(worry_level)
                    })
                },
                ["old", "+", v] => {
                    let value = v.parse::<usize>().or(Err(Day11Error::MonkeyParseError))?;
                    Box::new(move |worry_level| {
                        worry_level.wrapping_add(value)
                    })
                },
                ["old", "*", "old"] => {
                    Box::new(|worry_level| {
                        worry_level.wrapping_mul(worry_level)
                    })
                },
                ["old", "*", v] => {
                    let value = v.parse::<usize>().or(Err(Day11Error::MonkeyParseError))?;
                    Box::new(move |worry_level| {
                        worry_level.wrapping_mul(value)
                    })
                },
                _ => Box::new(|worry_level| worry_level),
            },
            throws_to: (true_throw, false_throw),
            divisible_by,
            inspection_count: 0,
        })
    }
}

impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let items = if self.items.is_empty() {
            "<no items>".to_string()
        } else {
            self.items.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", ")
        };
        write!(f, "Monkey {}: {}", self.id, items)
    }
}

fn parse_monkeys_from_file(file_path:&str) -> Vec<Monkey> {
    match read_to_string(&file_path) {
        Ok(content) => {
            let entries = content.lines().collect::<Vec<&str>>();
            entries
                .chunks(7)
                .map(|v| v.join("\n").parse::<Monkey>())
                .filter_map(|i| i.ok() )
                .collect::<Vec<Monkey>>()
        },
        Err(_) => Vec::new(),
    }
}

fn main()
{
    let args:Vec<String> = env::args().collect();
    if let None = args.get(1) {
        println!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }

    let mut monkeys = parse_monkeys_from_file(&args[1]);
    assert!(monkeys.len() > 0);

    // part2 we don't divide by 3, instead of have to find the least
    // common multiple of the divisible_by values
    let part2:usize = monkeys.iter().map(|m| m.divisible_by).product();

    for round in 0..10000 { // part 1 is 20 rounds, part 2 is 10000
        // first round
        for idx in 0..monkeys.len() {
            let m = &mut monkeys[idx];
            let throws:Vec<Throw> = m.throws(Some(part2));
            for throw in throws {
                let destination_monkey = &mut monkeys[throw.monkey];
                destination_monkey.catch(throw);
            }
        }

        println!("Round {}", round + 1);
        for monkey in &monkeys {
            println!("\t{}", monkey);
        }
    }
    for monkey in &monkeys {
        println!("Monkey {} inspected items {} times.", monkey.id, monkey.inspection_count);
    }
    let mut inspections:Vec<usize> = monkeys.iter().map(|m| -> usize {m.inspection_count}).collect::<Vec<usize>>();
    inspections.sort_by(|e1, e2| e2.cmp(&e1));
    let monkey_business:usize = inspections.iter().take(2).product();
    println!("{}", monkey_business);
}

#[cfg(test)]
mod tests {
    use super::{Monkey, Item, Throw};

    #[test]
    fn test_monkey() {
        let mut m = Monkey{
            id: 0,
            items: vec![Item::new(79), Item::new(98)],
            worry_level_mod: Box::new(|worry_level| worry_level * 19),
            throws_to: (2, 3),
            divisible_by: 23,
            inspection_count: 0,
        };
        assert_eq!(m.id, 0);
        assert_eq!(m.items, vec![Item::new(79), Item::new(98)]);
        assert_eq!((m.worry_level_mod)(3), 57);
        // part1 assert_eq!(m.throws(), vec![Throw::new(3, Item::new(500)), Throw::new(3, Item::new(620))]);
        assert_eq!(m.throws(Some(m.divisible_by)), vec![Throw::new(3, Item::new(6)), Throw::new(3, Item::new(22))]);

        assert!(m.items.is_empty());
        assert_eq!(m.inspection_count, 2);


        let m = "Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n".parse::<Monkey>().expect("failed to parse test case");
        assert_eq!((m.worry_level_mod)(1), 19); // old * 19

        let m = "Monkey 2:\n  Starting items: 79, 60, 97\n  Operation: new = old * old\n  Test: divisible by 13\n    If true: throw to monkey 1\n    If false: throw to monkey 3\n".parse::<Monkey>().expect("failed to parse test case");
        assert_eq!((m.worry_level_mod)(4), 16); // old * old
    }
}
