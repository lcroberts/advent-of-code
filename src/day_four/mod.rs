use std::{fs, io::BufRead};

#[inline]
fn check_right(width: usize, _height: usize, x: usize, y: usize, grid: &[Vec<char>]) -> bool {
    if x >= width - 3 {
        return false;
    }

    grid[y][x..=(x + 3)].iter().collect::<String>().eq("XMAS")
}

#[inline]
fn check_left(_width: usize, _height: usize, x: usize, y: usize, grid: &[Vec<char>]) -> bool {
    if x < 3 {
        return false;
    }

    grid[y][(x - 3)..=x]
        .iter()
        .rev()
        .collect::<String>()
        .eq("XMAS")
}

#[inline]
fn check_up(_width: usize, _height: usize, x: usize, y: usize, grid: &[Vec<char>]) -> bool {
    if y < 3 {
        return false;
    }
    (grid[y][x] == 'X')
        && (grid[y - 1][x] == 'M')
        && (grid[y - 2][x] == 'A')
        && (grid[y - 3][x] == 'S')
}

#[inline]
fn check_down(_width: usize, height: usize, x: usize, y: usize, grid: &[Vec<char>]) -> bool {
    if y >= height - 3 {
        return false;
    }

    (grid[y][x] == 'X')
        && (grid[y + 1][x] == 'M')
        && (grid[y + 2][x] == 'A')
        && (grid[y + 3][x] == 'S')
}

#[inline]
fn check_up_right(width: usize, _height: usize, x: usize, y: usize, grid: &[Vec<char>]) -> bool {
    if y < 3 || x >= width - 3 {
        return false;
    }

    (grid[y][x] == 'X')
        && (grid[y - 1][x + 1] == 'M')
        && (grid[y - 2][x + 2] == 'A')
        && (grid[y - 3][x + 3] == 'S')
}

#[inline]
fn check_up_left(_width: usize, _height: usize, x: usize, y: usize, grid: &[Vec<char>]) -> bool {
    if y < 3 || x < 3 {
        return false;
    }

    (grid[y][x] == 'X')
        && (grid[y - 1][x - 1] == 'M')
        && (grid[y - 2][x - 2] == 'A')
        && (grid[y - 3][x - 3] == 'S')
}

#[inline]
fn check_down_left(_width: usize, height: usize, x: usize, y: usize, grid: &[Vec<char>]) -> bool {
    if x < 3 || y >= height - 3 {
        return false;
    }

    (grid[y][x] == 'X')
        && (grid[y + 1][x - 1] == 'M')
        && (grid[y + 2][x - 2] == 'A')
        && (grid[y + 3][x - 3] == 'S')
}

#[inline]
fn check_down_right(width: usize, height: usize, x: usize, y: usize, grid: &[Vec<char>]) -> bool {
    if x >= width - 3 || y >= height - 3 {
        return false;
    }

    (grid[y][x] == 'X')
        && (grid[y + 1][x + 1] == 'M')
        && (grid[y + 2][x + 2] == 'A')
        && (grid[y + 3][x + 3] == 'S')
}

#[inline]
fn check_x(width: usize, height: usize, x: usize, y: usize, grid: &[Vec<char>]) -> bool {
    // Check bounds
    if x < 1 || x >= width - 1 || y < 1 || y >= height - 1 {
        return false;
    }
    if !grid[y][x].eq(&'A') {
        return false;
    }

    ((grid[y - 1][x - 1].eq(&'M') && grid[y + 1][x + 1].eq(&'S'))
        || (grid[y - 1][x - 1].eq(&'S') && grid[y + 1][x + 1].eq(&'M')))
        && ((grid[y + 1][x - 1].eq(&'M') && grid[y - 1][x + 1].eq(&'S'))
            || (grid[y + 1][x - 1].eq(&'S') && grid[y - 1][x + 1].eq(&'M')))
}

pub fn get_input() -> Vec<Vec<char>> {
    let input = fs::read("src/day_four/input").unwrap();
    let char_array: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    char_array
}

pub fn part_one(width: usize, height: usize, char_array: &[Vec<char>]) -> usize {
    let mut count = 0;
    for y in 0..height {
        for x in 0..width {
            if check_right(width, height, x, y, char_array) {
                count += 1;
            }
            if check_up_right(width, height, x, y, char_array) {
                count += 1;
            }
            if check_up(width, height, x, y, char_array) {
                count += 1;
            }
            if check_up_left(width, height, x, y, char_array) {
                count += 1;
            }
            if check_left(width, height, x, y, char_array) {
                count += 1;
            }
            if check_down_left(width, height, x, y, char_array) {
                count += 1;
            }
            if check_down(width, height, x, y, char_array) {
                count += 1;
            }
            if check_down_right(width, height, x, y, char_array) {
                count += 1;
            }
        }
    }
    count
}

pub fn part_two(width: usize, height: usize, char_array: &[Vec<char>]) -> usize {
    let mut count = 0;
    for y in 0..height {
        for x in 0..width {
            if check_x(width, height, x, y, char_array) {
                count += 1;
            }
        }
    }
    count
}

pub fn run() {
    let char_array = get_input();
    let width = char_array[0].len();
    let height = char_array.len();

    println!("Total XMAS count: {}", part_one(width, height, &char_array));
    println!(
        "Total X-MAS count: {}",
        part_two(width, height, &char_array)
    );
}
