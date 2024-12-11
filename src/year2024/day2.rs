use std::fs::File;
use std::i32;
use std::io::{BufRead, BufReader};

pub fn reports(){
    println!("day 2");
    let file = File::open("src/year2024/input2.txt").expect("failed");
    let reader = BufReader::new(file);
    let mut safe_reports: usize = 0;
    for line in reader.lines(){
        let line = line.expect("failed to read line");
        let report: Vec<i32> = line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect();
        let trend: bool = find_trend(&report);
        safe_reports += test_report(&report, trend, false);
    }
    println!("{safe_reports}")

}

//if increasing true else false
pub fn find_trend(report: &Vec<i32>) -> bool{
  if report[0] > report[1] {
      return false;
  }
  true
}
pub fn test_report(report: &Vec<i32>, trend: bool, second_iter:bool) -> usize
{    
let mut p1: usize = 0;
let mut p2: usize = 1;
while p2 < report.len(){
    let difference: i32;
    if trend{
        difference = report[p2] - report[p1]
    }
    else{
        difference = report[p1] -report[p2];
    }
    if difference <= 0 || difference > 3{
if second_iter{
    return 0
}
    return damper_check(&report, &p1, &p2);
    
    }
    p1 += 1;
    p2 += 1;
}
1
}
fn damper_check(report: &Vec<i32>, p1: &usize, p2: &usize) -> usize{
    let mut remove_from_p1 : Vec<i32> = report.clone();
    let mut remove_from_p2: Vec<i32> = report.clone();

    remove_from_p1.remove(*p1);
    remove_from_p2.remove(*p2);

    let first_trend = find_trend(&remove_from_p1);
    let first_check = test_report(&remove_from_p1, first_trend, true);

    let second_trend = find_trend(&remove_from_p2);
    let second_check = test_report(&remove_from_p2, second_trend, true);
    if first_check == 1 || second_check == 1 {
        return 1;
    }
    0

}
