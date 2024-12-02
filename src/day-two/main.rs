use std::fs;

fn main() {
    let input = fs::read_to_string("src/day-two/input").unwrap();
    let mut safe_count = 0;

    for line in input.split("\n") {
        if line.is_empty() {
            break;
        }
        let mut nums: Vec<u32> = Vec::new();
        for num in line.split(" ") {
            nums.push(num.parse::<u32>().unwrap());
        }
        println!("{:?}", nums);

        for i in 0..(nums.len() - 1) {}
    }
}
