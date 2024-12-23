use std::fs::File;
use std::io::prelude::*;
pub fn fix_instruction() {
    println!("day 3");
    let mut file = File::open("src/year2024/input3.txt").expect("failed to open a file");
    let mut instructions = String::new();
    let _ = file.read_to_string(&mut instructions);
    let total: usize = read_instructions(&instructions);
    println!("result is : {total}");
}

fn read_instructions(instructions: &String) -> usize {
    let instruction_vector: Vec<char> = instructions.chars().collect();
    let mut p1 = 0;
    let mut index = 0;
    let mut build = String::new();
    let mut sum = 0;
    let mut do_multiply: bool = true;
    while index < instruction_vector.len() {
        build.push(instruction_vector[index]);
        if build.ends_with("don't()") {
            do_multiply = false;
        }
        if build.ends_with("do()") {
            do_multiply = true;
        }

        if build.ends_with("mul(") && do_multiply {
            let mut a = String::new();
            let mut b = String::new();
            p1 = index + 1;
            while p1 < instruction_vector.len()
                && instruction_vector[p1].is_numeric()
                && a.len() < 3
            {
                a.push(instruction_vector[p1]);
                p1 += 1;
            }
            if p1 < instruction_vector.len() && instruction_vector[p1] == ',' {
                p1 += 1;
            } else {
                continue;
            }

            while p1 < instruction_vector.len()
                && instruction_vector[p1].is_numeric()
                && b.len() < 3
            {
                b.push(instruction_vector[p1]);
                p1 += 1;
            }

            if p1 < instruction_vector.len() && instruction_vector[p1] == ')' {
                if !a.is_empty() && !b.is_empty() {
                    println!("{} {}", a, b);
                    sum += a.parse::<usize>().unwrap_or(0) * b.parse::<usize>().unwrap_or(0);
                    println!("{sum}");
                }
            }
        }
        if p1 > index {
            index = p1 + 1;
        } else {
            index += 1;
        }
    }

    sum
}
