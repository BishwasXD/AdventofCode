use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn find_historian(){
println!("day1");
let mut file = File::open("src/year2024/input1.txt").expect("couldnt open the file");
let reader = BufReader::new(file);
let mut list1:Vec<i32> = Vec::new();
let  mut list2: Vec<i32> = Vec::new();
for line in reader.lines(){
    let line = line.expect("couldnt read a line");
    let splitline: Vec<&str> = line.split("   ").collect();
    list1.push(splitline[0].parse().expect("failed to parse"));
    list2.push(splitline[1].parse().expect("failed to parse"));  
}

let diff: i32 = find_difference(list1, list2);
println!("{diff}");

}

fn find_difference(mut list1: Vec<i32>, mut list2: Vec<i32>) -> i32{
list1.sort();
list2.sort();
let mut difference: i32 = 0;
for (index, _) in list1.iter().enumerate(){
    difference += (list1[index] - list2[index]).abs();

}
let score: i32 = similarity_score(list1, list2);
println!("{score}");
difference
}

fn similarity_score(list1:Vec<i32>, list2:Vec<i32>) -> i32 {

    let mut list2_frequency:HashMap<i32, i32> = HashMap::new();
    let mut similarity_score: i32 = 0;
    for (index,_) in list1.iter().enumerate(){

        list2_frequency.insert(list2[index], *list2_frequency.get(&list2[index]).unwrap_or(&0) + 1);

    }

    
    for id in list1.iter(){
        similarity_score += id * list2_frequency.get(&id).unwrap_or(&0);
    }
    
similarity_score
}

