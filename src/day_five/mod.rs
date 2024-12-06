use std::fs;

pub fn get_input() -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let input = fs::read_to_string("src/day_five/input").unwrap();

    let (ordering, updates) = input.split_at(input.find("\n\n").unwrap());
    let ordering: Vec<&str> = ordering.lines().collect();
    let updates: Vec<&str> = updates.lines().collect();
    let updates: Vec<&str> = updates[2..].into();

    let mut ordering_vec: Vec<(usize, usize)> = Vec::new();
    for line in ordering {
        let tmp = line
            .split("|")
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        ordering_vec.push((tmp[0], tmp[1]));
    }

    // let mut updates_vec: Vec<Vec<usize>> = Vec::new();
    let updates_vec: Vec<Vec<usize>> = updates
        .iter()
        .map(|line| {
            line.split(",")
                .map(|num| num.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    (ordering_vec, updates_vec)
}

pub fn part_one(ordering: Vec<(usize, usize)>, updates: Vec<Vec<usize>>) -> usize {
    0
}

pub fn run() {
    let (ordering, updates) = get_input();
    println!("Part One: {}", part_one(ordering, updates));
}
