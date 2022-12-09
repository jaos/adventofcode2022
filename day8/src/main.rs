/*
Treetop Tree House

Grid of tree heights:

[0]         [4]
 3  0  3  7  3
 2  5  5  1  2
 6  5  3  3  2
 3  3  5  4  9
 3  5  3  9  0

Tree is visible IF
    * all the other trees between it and an edge of the grid are SHORTER than it (rows AND columns).
    * the tree is not on an edge

Count the visible trees
*/
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct TreeGrid {
    columns: usize,
    rows: usize,
    data: Vec<usize>,
}
impl TreeGrid {
    fn get_row_for_idx(&self, idx:usize) -> Vec<&usize> {
        let row_start:usize = idx - idx % self.columns;
        self.data.as_slice()[row_start..row_start+self.columns].into_iter().collect()
    }

    fn get_column_for_idx(&self, idx:usize) -> Vec<&usize> {
        let col_idx = idx % self.columns;
        self.data.as_slice()[col_idx..].into_iter().step_by(self.columns).collect()
    }

    fn is_idx_visible_for_row_or_column(&self, idx:usize, value:usize) -> bool {
        let idx_for_row = idx % self.columns;
        let idx_for_col = idx / self.columns;
        let row = self.get_row_for_idx(idx);
        let column = self.get_column_for_idx(idx);

        let visible_from_top = column.as_slice()[..idx_for_col].into_iter().any(|&&i| i >= value);
        let visible_from_bottom = column.as_slice()[idx_for_col+1..].into_iter().any(|&&i| i >= value);
        let visible_from_left = row.as_slice()[..idx_for_row].into_iter().any(|&&i| i >= value);
        let visible_from_right = row.as_slice()[idx_for_row+1..].into_iter().any(|&&i| i >= value);
        visible_from_top || visible_from_bottom || visible_from_left || visible_from_right
    }

    fn is_idx_edge(&self, idx:usize) -> bool {
        (idx % self.columns) == 0 || (idx + 1) % self.columns == 0
    }

    fn idx_scenic_score(&self, idx:usize, value:usize) -> usize {
        let idx_for_row = idx % self.columns;
        let idx_for_col = idx / self.columns;
        let row = self.get_row_for_idx(idx);
        let column = self.get_column_for_idx(idx);

        let mut top_score = 0;
        for &e in column.as_slice()[..idx_for_col].into_iter().rev() { // revsered
            top_score += 1;
            if *e >= value { break; }
        }
        let mut bottom_score = 0;
        for &e in column.as_slice()[idx_for_col+1..self.columns].into_iter() {
            bottom_score += 1;
            if *e >= value { break; }
        }
        let mut left_score = 0;
        for &e in row.as_slice()[..idx_for_row].into_iter().rev() { // reversed
            left_score += 1;
            if *e >= value { break; }
        }
        let mut right_score = 0;
        for &e in row.as_slice()[idx_for_row+1..self.rows].into_iter() {
            right_score += 1;
            if *e >= value { break; }
        }
        top_score * bottom_score * left_score * right_score
    }

    fn find_best_scenic_score(&self) -> usize {
        self.data.iter().enumerate().map(|(idx, value)| self.idx_scenic_score(idx, *value) ).max().unwrap_or(0)
    }

    fn count_visible(&self) -> usize {
        let mut visible = 0;
        let last_row_idx = (self.columns * self.rows) - self.columns - 1;
        for (idx, value) in self.data.iter().enumerate() {
            // if we on the top or bottom row, or are edge, or is visible from a direction
            if idx < self.rows || idx >= last_row_idx || self.is_idx_edge(idx) || self.is_idx_visible_for_row_or_column(idx, *value) {
                visible += 1;
            }
        }
        visible
    }
}

fn parse_file(file_path:&str) -> Option<TreeGrid>
{
    let bf = BufReader::new(File::open(file_path).expect(file_path));
    let mut tree_grid = TreeGrid { columns: 0, rows: 0, data: Vec::new() };

    for line in bf.lines() {
        let grid_line = line.ok()?;
        tree_grid.columns = grid_line.len();
        tree_grid.rows += 1;
        for val in grid_line.as_str().chars().map(|c| c.to_digit(10).unwrap_or(0) as usize) {
            tree_grid.data.push(val);
        }
    }
    Some(tree_grid)
}

fn main()
{
    let args:Vec<String> = env::args().collect();
    if let None = args.get(1) {
        println!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }

    let tree_grid = parse_file(&args[1]).unwrap();
    dbg!(&tree_grid.count_visible());
    dbg!(&tree_grid.find_best_scenic_score());
}

#[cfg(test)]
mod tests {
    use super::TreeGrid;

    #[test]
    fn test_tree_grid() {
        let g = TreeGrid{columns: 5, rows: 5, data: vec![3,0,3,7,3, 2,5,5,1,2, 6,5,3,3,2, 3,3,5,4,9, 3,5,3,9,0]};
        assert_eq!(g.count_visible(), 25);
        assert!(g.is_idx_edge(0));
        assert!(g.is_idx_edge(4));
        assert!(!g.is_idx_edge(6));
        assert_eq!(g.get_row_for_idx(17), vec![&3,&3,&5,&4,&9]);
        assert_eq!(g.get_column_for_idx(17), vec![&3,&5,&3,&5,&3]);
        assert!(g.is_idx_visible_for_row_or_column(6, 5));
        assert_eq!(g.idx_scenic_score(7, 5), 4);
        assert_eq!(g.idx_scenic_score(17, 5), 8);
        assert_eq!(g.find_best_scenic_score(), 8);
    }
}
