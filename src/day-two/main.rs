use std::fs;

fn main() {
    let input = fs::read_to_string("src/day-two/input").unwrap();
    let mut safe_count = 0;

    for line in input.split("\n") {
        if line.is_empty() {
            break;
        }
        let mut nums: Vec<i64> = Vec::new();
        for num in line.split(" ") {
            nums.push(num.parse::<i64>().unwrap());
        }
        if check_safe(&nums) {
            safe_count += 1;
            continue;
        }

        for x in 0..nums.len() {
            let mut nums = nums.clone();
            nums.remove(x);
            if check_safe(&nums) {
                safe_count += 1;
                break;
            }
        }
    }
    println!("Safe Count: {safe_count}");
}

fn check_safe(nums: &[i64]) -> bool {
    let increasing = get_increasing(nums);
    let mut safe = true;
    for i in 0..(nums.len() - 1) {
        if !two_nums_safe(nums[i], nums[i + 1], increasing) {
            safe = false;
            break;
        }
    }
    safe
}

fn two_nums_safe(x: i64, y: i64, increasing: bool) -> bool {
    let mut curr_increasing = true;
    if x - y < 0 {
        curr_increasing = false;
    }
    if curr_increasing != increasing {
        return false;
    }
    if x == y || x.abs_diff(y) > 3 {
        return false;
    }
    true
}

fn get_increasing(nums: &[i64]) -> bool {
    nums[0] - nums[1] > 0
}
