use std::{char, fs};

pub fn run() {
    let do_str = "do()";
    let dont_str = "don't()";

    let input = fs::read_to_string("src/day_three/input").unwrap();
    let mut total = 0;
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    let mut mul_enabled = true;
    // 8 is the minimum valid mul substring length
    while i < (chars.len() - 7) {
        // Check a do
        let tmp: &String = &chars[i..=(i + do_str.len() - 1)].iter().collect();
        if tmp.eq(do_str) {
            i += do_str.len();
            mul_enabled = true;
            continue;
        }

        // Check a dont
        let tmp: &String = &chars[i..=(i + dont_str.len() - 1)].iter().collect();
        if tmp.eq(dont_str) {
            i += dont_str.len();
            mul_enabled = false;
            continue;
        }

        // At this point we try a mult
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
            i += 1;
        }
        if num1.is_empty() {
            i += 1;
            continue;
        }

        if chars[i] != ',' {
            i += 1;
            continue;
        }
        i += 1;

        let mut num2 = String::from("");
        while chars[i].is_numeric() {
            num2.push(chars[i]);
            i += 1;
        }
        if num2.is_empty() {
            i += 1;
            continue;
        }

        if chars[i] != ')' {
            i += 1;
            continue;
        }
        i += 1;

        // At this point we have a valid mul
        if mul_enabled {
            total += num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap();
        }
    }
    println!("Total After Muls: {total}");
}
