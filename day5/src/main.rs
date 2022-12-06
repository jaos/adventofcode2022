/* Supply Stacks */
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum ElfCrateErr {
    // InvalidOp(String), // if we ever need more than move
    Malformed,
    InvalidCount,
    InvalidSourceIndex,
    InvalidDestinationIndex,
    EmptyElfCrate,
}

/// Crate operation
/// ```
/// let op = ElfCrateOperation::from_string("move 3 from 2 to 1");
/// assert_eq!(op, ElfCrateOperation{count: 3, source_index: 2, destination_index: 1});
/// ```
#[derive(Debug, PartialEq, Eq)]
struct ElfCrateOperation {
    count: usize,
    source_index: usize,
    destination_index: usize,
}
impl FromStr for ElfCrateOperation {
    type Err = ElfCrateErr;
    fn from_str(v:&str) -> Result<Self, ElfCrateErr> {
        let op_parts:Vec<&str> = v.split_ascii_whitespace().collect();
        if op_parts.len() != 6 {
            return Err(ElfCrateErr::Malformed);
        }
        assert_eq!(op_parts.len(), 6);
        let count:usize = op_parts[1].parse().map_err(|_| ElfCrateErr::InvalidCount)?;
        let source_index:usize = op_parts[3].parse().map_err(|_| ElfCrateErr::InvalidSourceIndex)?;
        let destination_index:usize = op_parts[5].parse().map_err(|_| ElfCrateErr::InvalidDestinationIndex)?;
        Ok(Self{count, source_index, destination_index})
    }
}

#[derive(Debug, PartialEq, Eq)]
/// Elf crate stack
struct ElfCrateStack {
    crate_stacks: Vec<Vec<char>>,
}
impl ElfCrateStack {
    /// perform the operation on the elf crate stack
    fn perform_op(&mut self, op: ElfCrateOperation) -> Result<(), ElfCrateErr> {
        let num_of_crate_stacks = self.crate_stacks.len();
        if num_of_crate_stacks < op.source_index {
            return Err(ElfCrateErr::InvalidSourceIndex);
        }
        if num_of_crate_stacks < op.destination_index {
            return Err(ElfCrateErr::InvalidDestinationIndex);
        }
        // Part 1
        //for _ in 0..op.count {
            // let v = self.crate_stacks[op.source_index - 1].remove(0);
            // self.crate_stacks[op.destination_index - 1].insert(0, v);
        //}
        // Part 2
        let to_move:Vec<char> = self.crate_stacks[op.source_index - 1].drain(0..op.count).collect();
        for v in to_move.into_iter().rev() {
            self.crate_stacks[op.destination_index - 1].insert(0, v);
        }
        Ok(())
    }

    /// Return the top of all of the elf crate stacks as a string combination
    fn top_of_the_stacks(&self) -> Result<String, ElfCrateErr> {
        if self.crate_stacks.iter().any(|c| c.len() == 0) {
            return Err(ElfCrateErr::EmptyElfCrate);
        }
        let t:String = self.crate_stacks.iter().map(|c| c[0]).collect();
        Ok(t)
    }

    /// Parse the create header from the buffer, returning Vec<Vec<char>> of the crate labels
    ///
    ///     [D]    
    /// [N] [C]    
    /// [Z] [M] [P]
    ///  1   2   3 
    ///                    <-- header ends here
    /// move 1 from 2 to 1
    /// move 3 from 1 to 3
    /// move 2 from 2 to 1
    /// move 1 from 1 to 2
    ///
    /// This stops as soon as we get an empty line. The trailing number line is ignored.
    fn parse<R: Read>(f: &mut BufReader<R>) -> Result<Self, ElfCrateErr> {
        let mut elf_crate_stacks:Vec<Vec<char>> = Vec::new();
        for line in f.lines() {
            let entry = line.map_err(|_| ElfCrateErr::Malformed)?;

            // empty line is the end of the crate stack header in the file
            if entry.is_empty() {
                break;
            }

            let c:Vec<char> = entry.chars().collect();
            let chunks = c.chunks(4).map(|chunk|
                chunk.into_iter().filter(|c|
                     c.is_ascii_uppercase()
                ).next() // consume the option Some(<char>)
            );
            for (idx, crate_label) in chunks.enumerate() {
                // we don't know ahead of time how many stacks we care about... so we append the first time we see it
                if let None = elf_crate_stacks.get(idx) {
                    elf_crate_stacks.push(Vec::new());
                }
                if let Some(v) = crate_label {
                    elf_crate_stacks[idx].push(*v);
                }
            }
        }
        if elf_crate_stacks.is_empty() {
            return Err(ElfCrateErr::EmptyElfCrate);
        }
        Ok(ElfCrateStack { crate_stacks: elf_crate_stacks})
    }
}

fn day5(file_path:&str) -> Result<String, ElfCrateErr>
{
    let mut bf = BufReader::new(File::open(file_path).expect(file_path));
    // parse crate stack header of the file
    let mut elf_crate_stacks = ElfCrateStack::parse(&mut bf)?;

    // remaining lines are our crate operations
    for line in bf.lines() {
        let entry = line.map_err(|_| ElfCrateErr::Malformed)?;

        if let Ok(op) = entry.parse::<ElfCrateOperation>() {
            // perform op.count move operations
            elf_crate_stacks.perform_op(op)?;
        }
    }
    let top_of_the_stacks = elf_crate_stacks.top_of_the_stacks()?;
    Ok(top_of_the_stacks)
}

fn main()
{
    let args:Vec<String> = env::args().collect();
    if let None = args.get(1) {
        println!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }
    println!("{:?}", day5(&args[1]));
}

#[cfg(test)]
mod tests {
    use super::{ElfCrateErr, ElfCrateOperation, ElfCrateStack};
    use std::io::BufReader;

    #[test]
    fn test_crate_operation_parse() {
        assert_eq!("move 1 from 2 to 3".parse::<ElfCrateOperation>().ok(), Some(ElfCrateOperation{count: 1, source_index: 2, destination_index: 3}));
        assert!("move ZZZ from 2 to 3".parse::<ElfCrateOperation>().is_err());
        assert_eq!("move ZZZ from 2 to 3".parse::<ElfCrateOperation>().unwrap_err(), ElfCrateErr::InvalidCount);
        assert_eq!("move 1 from err to 3".parse::<ElfCrateOperation>().unwrap_err(), ElfCrateErr::InvalidSourceIndex);
        assert_eq!("move 1 from 2 to err".parse::<ElfCrateOperation>().unwrap_err(), ElfCrateErr::InvalidDestinationIndex);
        assert_eq!("garbage".parse::<ElfCrateOperation>().unwrap_err(), ElfCrateErr::Malformed);
    }

    static DATA:&str = concat!(
        "    [D]    \n",
        "[N] [C]    \n",
        "[Z] [M] [P]\n",
        " 1   2   3 \n",
        "\n",
        "move 1 from 2 to 1\n",
        "move 3 from 1 to 3\n",
        "move 2 from 2 to 1\n",
        "move 1 from 1 to 2\n"
    ); // defaults to NDP as a top of the stack

    #[test]
    fn test_elf_crate_stack_parse() {
        let mut bf = BufReader::new(DATA.as_bytes());
        let mut elf_crate_stacks = ElfCrateStack::parse(&mut bf).unwrap();
        dbg!(&elf_crate_stacks);
        println!("{:?}", elf_crate_stacks.top_of_the_stacks());
        assert_eq!(elf_crate_stacks.top_of_the_stacks().ok(), Some("NDP".to_string()));

        assert_eq!(elf_crate_stacks.perform_op("move 1 from 2 to 1".parse::<ElfCrateOperation>().unwrap()), Ok(()));
        assert_eq!(elf_crate_stacks.top_of_the_stacks().ok(), Some("DCP".to_string()));
        assert_eq!(elf_crate_stacks.perform_op("move 3 from 1 to 3".parse::<ElfCrateOperation>().unwrap()), Ok(()));
        assert_eq!(elf_crate_stacks.top_of_the_stacks().unwrap_err(), ElfCrateErr::EmptyElfCrate); // can't top b/c unbalanced
        assert_eq!(elf_crate_stacks.perform_op("move 2 from 2 to 1".parse::<ElfCrateOperation>().unwrap()), Ok(()));
        assert_eq!(elf_crate_stacks.perform_op("move 1 from 1 to 2".parse::<ElfCrateOperation>().unwrap()), Ok(()));
        assert_eq!(elf_crate_stacks.top_of_the_stacks().ok(), Some("MCD".to_string()));
    }
}
