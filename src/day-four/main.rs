use std::{fs, io::BufRead};

fn main() {
    let input = fs::read("src/day-four/input").unwrap();
    let char_array: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
}
