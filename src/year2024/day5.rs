use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

pub fn print_queue() {
    println!("day5");
    let file = File::open("src/year2024/input5.txt").expect("failed to read file");

    //use map to store first input since there will be duplicate key keep in vec key -> Vec<str> make index 1 key
    //again go through second input retrieve the value split and store in a var
    //loop throgh the var will be 2d loop for each val encountered check if that key exists in map
    //if exists we get a vec, loop and check if any val exists in the present row if exists we
    //check its index if less then current value(key value) rule has not violated.
    //if rule is 97|75 our map will be 75 -> [97,..] and input is 75,97 when at 75 we get the vec
    //containing 97 and we check in 75,97 where 97 occurs it is 1, the other index is 0 for 75
    //acc to rule 97 should be ahead of 75 but 1 > 0 rule is violated

    let reader = BufReader::new(file);
    let mut page_rule: HashMap<String, Vec<String>> = HashMap::new();
    let mut add_page_number = 0;
    let mut page_info: Vec<Vec<String>> = Vec::new();
    let mut end_detected: bool = false;
    for line in reader.lines() {
        let line = line.expect("failes to read line");
        if line.is_empty() {
            end_detected = true;
            continue;
        }
        if !end_detected {
            let pages: Vec<&str> = line.split('|').collect();
            page_rule
                .entry(pages[1].to_string())
                .and_modify(|vec| vec.push(pages[0].to_string()))
                .or_insert(vec![pages[0].to_string()]);
        } else {
            let pages: Vec<String> = line.split(',').map(|s| s.to_string()).collect();
            page_info.push(pages);
        }
    }
    for i in 0..page_info.len() {
        let mut do_addition = true;
        for j in 0..page_info[i].len() {
            let get_from_map = page_rule.get(&page_info[i][j]);

            match get_from_map {
                Some(vals) => {
                    for val in vals {
                        if page_info[i].contains(val) {
                            if let Some(index) = page_info[i].iter().position(|s| s == val) {
                                if j < index {
                                    do_addition = false;
                                    page_info[i].remove(index);
                                    page_info[i].insert(j, val.to_string());
                                }
                            }
                        }
                    }
                }
                None => println!("do nothing"),
            }
        }
        if !do_addition {
            let middle_index = (page_info[i].len() - 1) / 2;
            add_page_number += page_info[i][middle_index]
                .parse::<u32>()
                .expect("failed to parse");
        }
    }
    println!("{add_page_number}");
    // println!("{page_info:?}"); }
}
