/*
Distress signal

Pairs of packets (separated by a blank line)

[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]

*/
use std::env;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum Day13Error {
    TokenParseError,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Token {
    Number(u32),
    Comma,
    StartList,
    EndList,
}

impl FromStr for Token {
    type Err = Day13Error;
    fn from_str(value:&str) -> Result<Self, Day13Error> {
        match value.parse::<u32>() {
            Ok(n) => Ok(Token::Number(n)),
            Err(_) => match value {
                "," => Ok(Token::Comma),
                "]" => Ok(Token::EndList),
                "[" => Ok(Token::StartList),
                _ => Err(Day13Error::TokenParseError),
            }
        }
    }
}

#[derive(Debug)]
struct PacketScanner<'a> {
    data: &'a str,
    start: usize,
    current: usize,
    tokens: Vec<Token>,
}
impl<'a> PacketScanner<'a> {
    fn new(input:&'a str) -> Self {
        let mut me = Self{data:input, start:0, current:0, tokens: vec![]};

        while !me.at_end() {
            me.start = me.current;
            me.scan();
        }
        me
    }

    fn at_end(&self) -> bool {
        self.current == self.data.len()
    }

    fn advance(&mut self) -> Option<char> {
        let v = self.peek();
        match &v {
            Some(_) => {
                self.current += 1;
                v
            },
            None => v
        }
    }
    fn peek(&self) -> Option<char> {
        if self.at_end() {
            None
        } else {
            Some(self.data.as_bytes()[self.current] as char)
        }
    }

    fn consume_number(&mut self) {
        loop {
            match self.peek() {
                Some(v) => {
                    if !v.is_digit(10) {
                        break;
                    } else {
                        self.advance();
                    }
                },
                None => break,
            }
        }
        if let Ok(token) = self.data[self.start..self.current].parse::<Token>() {
            self.tokens.push(token);
        }
    }

    fn scan(&mut self) {
        match self.advance() {
            Some(']') => self.tokens.push(Token::EndList),
            Some('[') => self.tokens.push(Token::StartList),
            Some(',') => self.tokens.push(Token::Comma),
            Some(v) if v.is_digit(10) => self.consume_number(),
            Some(_) => todo!("unexpected token?"),
            None => {},
        }
    }

    fn tokenizer(input:&'a str) -> Vec<Token> {
        Self::new(input).tokens
    }
}

type Packets = Vec<Packet>;
#[derive(Debug, PartialEq, Eq, Ord)]
enum Packet {
    Number(u32),
    List(Packets),
}

// I think this is where I finally said wow, i love rust
impl PartialOrd for Packet {
    fn partial_cmp(&self, other:&Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Packet::Number(s), Packet::Number(o)) => (*s).partial_cmp(o),
            (Packet::Number(s), Packet::List(_)) => Packet::List(vec![Packet::Number(*s)]).partial_cmp(other),
            (Packet::List(s), Packet::List(o)) => (*s).partial_cmp(o),
            (Packet::List(_), Packet::Number(o)) => (*self).partial_cmp(&Packet::List(vec![Packet::Number(*o)])),
        }
    }
}

impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Number(n) => {
                write!(f, "{}", n)
            },
            Packet::List(l) => {
                let list_str = format!("[{}]", l.into_iter().map(|i| format!("{}", i)).collect::<Vec<String>>().join(","));
                write!(f, "{}", list_str)
            },
        }
    }
}


#[derive(Debug)]
struct PacketParser {
    tokens: Vec<Token>,
    current: usize,
    packet: Packet,
}

impl PacketParser {
    fn new(tokens:Vec<Token>) -> Self {
        // NOTE assumption: we always start with a list Packet
        let mut me = Self{tokens, current:0, packet: Packet::List(vec![])};
        assert_eq!(me.advance(), Some(Token::StartList));

        while !me.at_end() {
            me.scan();
        }
        me
    }

    fn at_end(&self) -> bool {
        self.current == self.tokens.len()
    }

    fn advance(&mut self) -> Option<Token> {
        let v = self.peek();
        match &v {
            Some(_) => {
                self.current += 1;
                v
            },
            None => v,
        }
    }

    fn peek(&self) -> Option<Token> {
        if self.at_end() {
            None
        } else {
            Some(self.tokens[self.current])
        }
    }

    fn consume_list(&mut self) -> Packet {
        let mut list = Packet::List(vec![]);
        loop {
            match self.peek() {
                Some(Token::Number(n)) => {
                    if let Packet::List(v) = &mut list {
                        v.push(Packet::Number(n));
                    }
                    self.advance(); // toss it
                },
                Some(Token::Comma) => {
                    self.advance(); // toss it
                },
                Some(Token::StartList) => { // recurse
                    self.advance(); // toss it
                    let sublist = self.consume_list();
                    if let Packet::List(v) = &mut list {
                        v.push(sublist);
                    }
                },
                Some(Token::EndList) => {
                    self.advance(); // toss it
                    break;
                },
                None => break,
            }
        }
        list
    }

    fn scan(&mut self) {
        match self.advance() {
            Some(Token::Number(n)) => {
                if let Packet::List(v) = &mut self.packet {
                    v.push(Packet::Number(n));
                }
            },
            Some(Token::StartList) => {
                let sublist = self.consume_list();
                if let Packet::List(v) = &mut self.packet {
                    v.push(sublist);
                }
            },
            Some(Token::EndList) => (), // the end
            Some(Token::Comma) => (), // ignore
            None => {},
        }
    }

    fn parse(tokens:Vec<Token>) -> Packet {
        Self::new(tokens).packet
    }
}

fn main()
{
    let args:Vec<String> = env::args().collect();
    if let None = args.get(1) {
        println!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }

    let mut counter = 1;
    let mut part1_index_in_order_sum = 0;
    let mut part2:Vec<Packet> = Vec::new();
    if let Ok(data) = read_to_string(&args[1]) {
        for pair in data.split("\n\n") {
            if let Some((first, second)) = pair.trim().split_once("\n") {
                let one = PacketParser::parse(PacketScanner::tokenizer(first));
                let two = PacketParser::parse(PacketScanner::tokenizer(second));

                // if in order, count it
                if one < two {
                    part1_index_in_order_sum += counter;
                }
                part2.push(one);
                part2.push(two);
            }
            counter += 1;
        }
    }
    println!("Part1: {}", part1_index_in_order_sum);

    // part2 divider packets
    part2.push(PacketParser::parse(PacketScanner::tokenizer("[[2]]")));
    part2.push(PacketParser::parse(PacketScanner::tokenizer("[[6]]")));

    part2.sort();

    let (mut part2_divider_idx1, mut part2_divider_idx2) = (0,0);
    for (idx, packet) in part2.iter().enumerate() {
        println!("{} {}", idx, packet);
        if packet.to_string() == "[[2]]" {
            part2_divider_idx1 = idx + 1;
        }
        if packet.to_string() == "[[6]]" {
            part2_divider_idx2 = idx + 1;
        }
    }
    println!("Part2: {}", part2_divider_idx1 * part2_divider_idx2);

}

#[cfg(test)]
mod tests {
    use super::{Day13Error, Packet, PacketParser, PacketScanner, Token};

    #[test]
    fn test_token() {
        assert_eq!("1".parse::<Token>(), Ok(Token::Number(1)));
        assert_eq!("[".parse::<Token>(), Ok(Token::StartList));
        assert_eq!("]".parse::<Token>(), Ok(Token::EndList));
        assert_eq!(",".parse::<Token>(), Ok(Token::Comma));
        assert_eq!("/".parse::<Token>(), Err(Day13Error::TokenParseError));
    }
    #[test]
    fn test_packetscanner() {
        assert_eq!(PacketScanner::tokenizer("[1,2]"), vec![
            Token::StartList,
            Token::Number(1),
            Token::Comma,
            Token::Number(2),
            Token::EndList,
        ]);
        assert_eq!(PacketScanner::tokenizer("[[1],2]"), vec![
            Token::StartList,
            Token::StartList,
            Token::Number(1),
            Token::EndList,
            Token::Comma,
            Token::Number(2),
            Token::EndList,
        ]);
        assert_eq!(PacketScanner::tokenizer("[[[[]]]]"), vec![
            Token::StartList,
            Token::StartList,
            Token::StartList,
            Token::StartList,
            Token::EndList,
            Token::EndList,
            Token::EndList,
            Token::EndList,
        ]);
        assert_eq!(PacketScanner::tokenizer("[[[[8,6,7],9,7,10],[[2,2,4],0,[4,9,10],[4,8,1,1],9],5],[8,8,[5,7],1,3],[],[6,[1,[0,1],[6,10,9]]],[]]"), vec![Token::StartList, Token::StartList, Token::StartList, Token::StartList, Token::Number(8), Token::Comma, Token::Number(6), Token::Comma, Token::Number(7), Token::EndList, Token::Comma, Token::Number(9), Token::Comma, Token::Number(7), Token::Comma, Token::Number(10), Token::EndList, Token::Comma, Token::StartList, Token::StartList, Token::Number(2), Token::Comma, Token::Number(2), Token::Comma, Token::Number(4), Token::EndList, Token::Comma, Token::Number(0), Token::Comma, Token::StartList, Token::Number(4), Token::Comma, Token::Number(9), Token::Comma, Token::Number(10), Token::EndList, Token::Comma, Token::StartList, Token::Number(4), Token::Comma, Token::Number(8), Token::Comma, Token::Number(1), Token::Comma, Token::Number(1), Token::EndList, Token::Comma, Token::Number(9), Token::EndList, Token::Comma, Token::Number(5), Token::EndList, Token::Comma, Token::StartList, Token::Number(8), Token::Comma, Token::Number(8), Token::Comma, Token::StartList, Token::Number(5), Token::Comma, Token::Number(7), Token::EndList, Token::Comma, Token::Number(1), Token::Comma, Token::Number(3), Token::EndList, Token::Comma, Token::StartList, Token::EndList, Token::Comma, Token::StartList, Token::Number(6), Token::Comma, Token::StartList, Token::Number(1), Token::Comma, Token::StartList, Token::Number(0), Token::Comma, Token::Number(1), Token::EndList, Token::Comma, Token::StartList, Token::Number(6), Token::Comma, Token::Number(10), Token::Comma, Token::Number(9), Token::EndList, Token::EndList, Token::EndList, Token::Comma, Token::StartList, Token::EndList, Token::EndList]);
    }

    #[test]
    fn test_packetparser() {
        assert_eq!(PacketParser::parse(PacketScanner::tokenizer("[10,20,30]")),
            Packet::List(vec![
                Packet::Number(10),
                Packet::Number(20),
                Packet::Number(30),
            ])
        );

        assert_eq!(PacketParser::parse(PacketScanner::tokenizer("[11,[22,[33]]]")),
            Packet::List(vec![
                Packet::Number(11),
                Packet::List(vec![
                    Packet::Number(22),
                    Packet::List(vec![
                        Packet::Number(33),
                    ]),
                ]),
            ])
        );
        assert_eq!(PacketParser::parse(PacketScanner::tokenizer("[[[[8,6,7],9,7,10],[[2,2,4],0,[4,9,10],[4,8,1,1],9],5],[8,8,[5,7],1,3],[],[6,[1,[0,1],[6,10,9]]],[]]")),
            Packet::List(vec![
                Packet::List(vec![
                    Packet::List(vec![
                        Packet::List(vec![Packet::Number(8), Packet::Number(6), Packet::Number(7)]),
                        Packet::Number(9),
                        Packet::Number(7),
                        Packet::Number(10)
                    ]),
                    Packet::List(vec![
                        Packet::List(vec![
                            Packet::Number(2),
                            Packet::Number(2),
                            Packet::Number(4),
                        ]),
                        Packet::Number(0),
                        Packet::List(vec![
                            Packet::Number(4),
                            Packet::Number(9),
                            Packet::Number(10),
                        ]),
                        Packet::List(vec![
                            Packet::Number(4),
                            Packet::Number(8),
                            Packet::Number(1),
                            Packet::Number(1),
                        ]),
                        Packet::Number(9),
                    ]),
                    Packet::Number(5),
                ]),
                Packet::List(vec![
                    Packet::Number(8),
                    Packet::Number(8),
                    Packet::List(vec![
                        Packet::Number(5),
                        Packet::Number(7),
                    ]),
                    Packet::Number(1),
                    Packet::Number(3),
                ]),
                Packet::List(vec![
                ]),
                Packet::List(vec![
                    Packet::Number(6),
                    Packet::List(vec![
                        Packet::Number(1),
                        Packet::List(vec![
                            Packet::Number(0),
                            Packet::Number(1),
                        ]),
                        Packet::List(vec![
                            Packet::Number(6),
                            Packet::Number(10),
                            Packet::Number(9),
                        ]),
                    ]),
                ]),
                Packet::List(vec![
                ]),
            ])
        );
    }

    #[test]
    fn test_packet() {
        let less_than = [
            ("[]", "[3]"),
            ("[1,1,3,1,1]", "[1,1,5,1,1]"),
            ("[[1],[2,3,4]]", "[[1],4]"),
            ("[[4,4],4,4]", "[[4,4],4,4,4]"),
            ("[[4,[[]]],[],[[1]],[]]",
                 "[[5,[[7,3,2,4]],3,[]],[[[4,5,3],6,10,[5,3,3],5],[[],10,1],3,[9,[8,9,2]]],[1],[3,[[5,3,9,5],[8,6,2,7]],[],2,2]]"),
            ("[[2,[],[]],[[[3,2,8],7,10,10],[[],[],[0,8],[8,6,7,6,3]],9,[10,[2,3,9],0]],[4,10,[],7]]", "[[2,[5,[4,5],1,[5,3,6,0,10]]],[[],[0,10,[4],[]],6,4,[6,[0,7,1],[3,8],[4,9,4,2]]]]"),
        ];
        for (first, second) in less_than {
            assert!(
                PacketParser::parse(PacketScanner::tokenizer(first))
                < PacketParser::parse(PacketScanner::tokenizer(second)),
                "Failed lt check for\n{:?}\n{:?}", first, second
            );
            assert!(
                PacketParser::parse(PacketScanner::tokenizer(second))
                > PacketParser::parse(PacketScanner::tokenizer(first)),
                "Failed lt(2) check for\n{:?}\n{:?}", first, second
            );
        }
        let greater_than = [
            ("[[[]]]", "[[]]"),
            ("[1,1,5,1,1]", "[1,1,3,1,1]"),
            ("[9]", "[[8,7,6]]"),
            ("[7,7,7,7]", "[7,7,7]"),
            ("[7,77,7]", "[7,7,7]"),
            ("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]"),
            ("[[[4,[5,10,6,7,6],10,5,6],[4],[],2,5],[],[]]", "[[[0,[8,9,5,0],4]],[[8,2,7,1,[2,3,4]],4,[]]]"),
            ("[[5,[[0],6,[7,8,7,5],[4,8,7,7],10],[0,9,[4,9,9,6,3],[6],4],[[8,9],3,[]]],[[]],[0,[[10,10,5,8,5],2,7,0,[3]],[2,[4,6,5,1,6],[7,10,10,4],7]]]", "[[5,0],[[[6,3,5],[3],[8,1,5],5,9],[6],[2,0],2,[10]],[]]"),

        ];
        for (first, second) in greater_than {
            assert!(
                PacketParser::parse(PacketScanner::tokenizer(first))
                > PacketParser::parse(PacketScanner::tokenizer(second)),
                "Failed gt check for\n{:?}\n{:?}", first, second
            );
            assert!(
                PacketParser::parse(PacketScanner::tokenizer(second))
                < PacketParser::parse(PacketScanner::tokenizer(first)),
                "Failed gt(2) check for\n{:?}\n{:?}", first, second
            );
        }


        let stringify_inputs = [
            "[[5,[[0],6,[7,8,7,5],[4,8,7,7],10],[0,9,[4,9,9,6,3],[6],4],[[8,9],3,[]]],[[]],[0,[[10,10,5,8,5],2,7,0,[3]],[2,[4,6,5,1,6],[7,10,10,4],7]]]",
            "[[5,0],[[[6,3,5],[3],[8,1,5],5,9],[6],[2,0],2,[10]],[]]",
            "[[[[8,6,7],9,7,10],[[2,2,4],0,[4,9,10],[4,8,1,1],9],5],[8,8,[5,7],1,3],[],[6,[1,[0,1],[6,10,9]]],[]]",
        ];
        for input in stringify_inputs {
            let input_str = format!("{}", PacketParser::parse(PacketScanner::tokenizer(input)));
            assert_eq!(input, input_str);
        }
    }
}
