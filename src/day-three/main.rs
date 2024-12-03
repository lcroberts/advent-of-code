use std::{char, fs};

fn main() {
    let input = fs::read_to_string("src/day-three/input").unwrap();
    let mut total = 0;
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    // 8 is the minimum valid mul substring length
    while i < (chars.len() - 3) {
        // TODO: Fix infinite loop in here
        println!("i: {i}");
        let mul: &String = &chars[i..=(i + 2)].iter().collect();
        if !mul.eq("mul") {
            i += 1;
            continue;
        }

        i += 3;
        if chars[i] != '(' {
            i += 1;
            continue;
        }
        i += 1;

        let mut num1 = String::from("");
        while chars[i].is_numeric() {
            num1.push(chars[i]);
        }
        if num1.is_empty() {
            i += 1;
            continue;
        }
        i += num1.len();

        if chars[i] != ',' {
            i += 1;
            continue;
        }
        i += 1;

        let mut num2 = String::from("");
        while chars[i].is_numeric() {
            num2.push(chars[i]);
        }
        if num2.is_empty() {
            i += 1;
            continue;
        }
        i += num2.len();
        if chars[i] != ')' {
            i += 1;
            continue;
        }
        i += 1;

        // At this point we have a valid mul
        total += num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap();
    }
    println!("Total After Muls: {total}");
}
