use std::{collections::HashMap, fs, vec};

fn main() {
    let contents = fs::read_to_string("src/day-one/input").unwrap();
    let mut left_nums: Vec<u32> = vec![];
    let mut right_nums: Vec<u32> = vec![];
    let mut right_counts: HashMap<u32, u32> = HashMap::new();
    for substr in contents.split("\n") {
        let nums = substr.split(" ");
        let nums: Vec<_> = nums.collect();
        if nums[0].is_empty() {
            break;
        }
        let left = nums[0].parse::<u32>().unwrap();
        let right = nums[1].parse::<u32>().unwrap();
        left_nums.push(left);
        right_nums.push(right);

        let curr = right_counts.get(&right).unwrap_or(&0);
        right_counts.insert(right, *curr + 1);
    }
    left_nums.sort();
    right_nums.sort();

    let mut diff = 0;
    for i in 0..left_nums.len() {
        diff += left_nums[i].abs_diff(right_nums[i]);
    }
    println!("Total Distance: {diff}");

    let mut similarity_score = 0;
    for x in left_nums {
        let count = *(right_counts.get(&x).unwrap_or(&0));
        similarity_score += count * x;
    }

    println!("Similarity Score: {similarity_score}");
}
