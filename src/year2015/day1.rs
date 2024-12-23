use std::fs::File;
use std::io::prelude::*;
pub fn day_one() {
    let mut file = File::open("src/year2015/day1.txt").expect("cant open the file");
    let mut instruction = String::new();
    let mut count = 0;
    file.read_to_string(&mut instruction)
        .expect("cant read file contents");
    let length = instruction.len();
    println!("instruction is : {}", instruction);
    // for ins in instruction.chars(){
    //     if ins == '('{
    //       count += 1;
    //     }
    //     else if ins == ')' {
    //         count -= 1;
    //     }
    // }

    for (index, c) in instruction.chars().enumerate() {
        if count != -1 {
            if c == '(' {
                count += 1;
            } else if c == ')' {
                count -= 1;
            }
        } else {
            println!("basement reached : {index}");
            break;
        }
    }
}
