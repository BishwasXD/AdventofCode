use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn intern_elves(){
    println!("day 5");
    let mut file = File::open("src/year2015/input5.txt").expect("couldn't open the file");
    let buf = BufReader::new(&file);
    let mut nice_count: usize = 0;
    for line in buf.lines(){
        let line = line.expect("couldn't read");
        nice_count += find_nice(&line);     
    }
    println!("{nice_count}");
}

pub fn find_nice(letter: &str) -> usize{
    //if nice return 1 else return 0;
    let mut reconstruct = String::new();
    let _vowel = "aeiou";
    let mut pairset:HashSet<(char, char)> = HashSet::new();
    let mut _vowel_count: u16 = 0;
    let _naughty_substrings: Vec<&str> = vec!["ab","cd", "pq", "xy"];
    let mut criteria_fulfilled: u16 = 0;
    let mut sl: usize = 0;
     for ch in letter.chars(){
    //     if vowel.contains(ch){
    //         vowel_count += 1;
    //     }
        reconstruct.push(ch);

        //refactored for part 2(doesn't works)
        if reconstruct.len() > 2{
            let split_index = reconstruct.len() - 3;
            let splitted = &reconstruct[split_index..];
            println!("{splitted:?}");
            //if naughty_substrings.contains(&splitted){
              //  return 0;
            //}

            let stov:Vec<char> = splitted.chars().collect();
            if stov[0] == stov[2]{
                criteria_fulfilled = 1;
    }
            if stov[0] == stov[1] && stov[1] == stov[2]{
                sl += 1;
            }
            pairset.insert((stov[0], stov[1]));
            pairset.insert((stov[1],stov[2]));
        }
     }
       
        // if vowel_count > 2{
        //     criteria_fulfilled += 1;
        // }
        let set_len = pairset.len() + sl;
        let letter_len = letter.len();
        if set_len != letter_len - 1{
            criteria_fulfilled += 1;
        }
        println!("{criteria_fulfilled}");
        if criteria_fulfilled == 2{
            return 1
        }

        0
}
