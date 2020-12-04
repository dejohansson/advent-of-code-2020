use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    let f = BufReader::new(fs::File::open("data.txt").unwrap());
    let grid: Vec<Vec<char>> = f
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .collect();

    let mut product = 1;
    for &(x, y) in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter() {
        let num_trees = test_slope(&grid, x, y);
        product *= num_trees;
        println!("Encountered {} trees with slope ({},{})", num_trees, x, y);
    }
    println!("Result: {}", product);
}

fn test_slope(grid: &Vec<Vec<char>>, x_offset: usize, y_offset: usize) -> usize {
    const TREE: char = '#';
    let mut num_trees = 0;
    let width = grid[0].len();

    for (y, row) in grid.iter().step_by(y_offset).enumerate() {
        let x = y * x_offset % width;
        if row[x] == TREE {
            num_trees += 1;
        }
    }

    num_trees
}
