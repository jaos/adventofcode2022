/* No space left on device

$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k


- / (dir)
  - a (dir)
    - e (dir)
      - i (file, size=584)
    - f (file, size=29116)
    - g (file, size=2557)
    - h.lst (file, size=62596)
  - b.txt (file, size=14848514)
  - c.dat (file, size=8504156)
  - d (dir)
    - j (file, size=4060174)
    - d.log (file, size=8033020)
    - d.ext (file, size=5626152)
    - k (file, size=7214296)

To begin, find all of the directories with a total size of at most 100000, then calculate the sum of their total sizes. In the example above, these directories are a and e; the sum of their total sizes is 95437 (94853 + 584). (As in this example, this process can count files more than once!)

Find all of the directories with a total size of at most 100000. What is the sum of the total sizes of those directories?

*/
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::collections::HashMap;

static ROOT_PATH:&str = "/";
static CD_CMD:&str = "$ cd ";
static CD_PARENT:&str = "..";
static DIR_PREFIX:&str = "dir ";
static LS_CMD:&str = "$ ls";

#[derive(Debug)]
struct Day7Dir {
    path: PathBuf,
    files: HashMap<PathBuf, usize>,
    directories: HashMap<PathBuf, Day7Dir>,
}
impl Day7Dir {
    fn new(path: &PathBuf) -> Self{
        Self { path: path.clone(), files: HashMap::new(), directories: HashMap::new()}
    }
    fn total(&self) -> usize {
        self.files.iter().map(|(_, s)| s).sum::<usize>() + self.directories.iter().map(|(_, d)| d.total()).sum::<usize>()
    }

}

/*
TODO
    let root_iter = Day7DirIter::new(&root);
    for dir in root_iter {
        println!("{:?}", dir.path);
    }
*/
/*
struct Day7DirIter <'d> {
    data: &'d Day7Dir,
    didself: bool,
    iter: Iter<'d, PathBuf, Day7Dir>,
}

impl<'a> Day7DirIter<'a> {
    fn new(data: &'a Day7Dir) -> Self {
        Self { data: data, didself: false, iter: data.directories.iter()}
    }
}

impl<'i> Iterator for Day7DirIter<'i> {
    type Item = &'i Day7Dir;

    fn next(&mut self) -> Option<Self::Item> {
        match self.didself {
            true => {
                match self.iter.next() {
                    Some((_, dir)) => Some(dir),
                    None => None,
                }
            },
            false => {
                self.didself = true;
                Some(self.data)
            }
        }
    }
}
*/

fn find_dir<'a>(root:&'a mut Day7Dir, path:&PathBuf) -> Option<&'a mut Day7Dir> {
    if root.path == *path {
        return Some(root);
    } else {
        for (_, dir) in &mut root.directories {
            match find_dir(dir, path) {
                Some(d) => {return Some(d)},
                None => {},
            }
        }
    }
    None
}

fn find_big_ones<'a>(root:&'a Day7Dir, collector:&mut Vec<&'a Day7Dir>)
{
    if root.total() <= 100000 {
        collector.push(root);
    }
    for (_, dir) in &root.directories {
        find_big_ones(&dir, collector);
    }
}

fn find_delete_candidates<'a>(root:&'a Day7Dir, collector:&mut Vec<&'a Day7Dir>, threshhold:usize)
{
    if root.total() >= threshhold {
        collector.push(root);
    }
    for (_, dir) in &root.directories {
        find_delete_candidates(&dir, collector, threshhold);
    }
}

fn resolve_path(cwd:&mut PathBuf, op:&str)
{
    if let Some(cd_path) = op.split_ascii_whitespace().nth(2) {
        if cd_path == CD_PARENT {
            cwd.pop();
        } else {
            cwd.push(cd_path);
        }
    }
}

fn day7(file_path:&str) -> Option<(usize, usize)>
{
    let bf = BufReader::new(File::open(file_path).expect(file_path));

    let mut current_path = PathBuf::from(ROOT_PATH);
    let mut root = Day7Dir::new(&current_path);

    for line in bf.lines() {
        let entry = line.ok()?;
        if entry.starts_with(&CD_CMD) {
            resolve_path(&mut current_path, &entry);
        } else if entry.starts_with(&DIR_PREFIX) {
            let parts:Vec<&str> = entry.split(" ").collect();
            let subdir = current_path.join(parts[1]);
            if let Some(that_dir) = find_dir(&mut root, &current_path) {
                that_dir.directories.insert(subdir.clone(), Day7Dir::new(&subdir));
            }

        } else if entry.starts_with(&LS_CMD) {
            continue;
        } else {
            match entry.split_once(" ") {
                Some((file_size, file_name)) => {
                    if let Ok(file_size_value) = file_size.parse::<usize>() {
                        if let Some(that_dir) = find_dir(&mut root, &current_path) {
                            let file_path = current_path.join(file_name);
                            //println!("Inserting file {:?} into {:?}", &file_path, &that_dir);
                            that_dir.files.insert(file_path, file_size_value);
                        }
                    }
                },
                None => {},
            }
        }
    }

    let mut big_ones:Vec<&Day7Dir> = Vec::new();
    find_big_ones(&root, &mut big_ones);
    let total_size_of_big_ones = big_ones.iter().map(|&d| d.total()).sum::<usize>();

    println!("root size is {}", root.total());
    println!("unused space {}", 70000000 - root.total());
    let space_required = 30000000 - (70000000 - root.total());
    println!("need additional space freed up {}", space_required);

    let mut big_ones:Vec<&Day7Dir> = Vec::new();
    find_delete_candidates(&root, &mut big_ones, space_required);
    big_ones.sort_by_key(|d| d.total());
    for f in &big_ones {
        println!("candidate {:?} {} (target {})", f.path, f.total(), space_required);
    }
    // dbg!(big_ones.iter().filter(|&d| d.total() >= 30000000).collect::<Vec<&Day7Dir>>());

    Some((total_size_of_big_ones, big_ones[0].total()))
}

fn main()
{
    let args:Vec<String> = env::args().collect();
    if let None = args.get(1) {
        println!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }
    println!("{:?}", day7(&args[1]));
}


#[cfg(test)]
mod tests {
    use super::{ROOT_PATH, resolve_path};
    use std::path::PathBuf;

    #[test]
    fn test_resolve_path() {
        let mut p = PathBuf::from(ROOT_PATH);

        resolve_path(&mut p, "$ cd ..");
        assert_eq!(p.to_str(), Some("/"));

        resolve_path(&mut p, "$ cd foo");
        assert_eq!(p.to_str(), Some("/foo"));

        resolve_path(&mut p, "$ cd bar");
        assert_eq!(p.to_str(), Some("/foo/bar"));

        resolve_path(&mut p, "$ cd ..");
        assert_eq!(p.to_str(), Some("/foo"));

        resolve_path(&mut p, "$ cd /");
        assert_eq!(p.to_str(), Some("/"));
    }
}
