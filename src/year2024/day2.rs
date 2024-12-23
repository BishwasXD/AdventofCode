use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn reports() {
    println!("day 2");
    let file = File::open("src/year2024/input2.txt").expect("failed");
    let reader = BufReader::new(file);

    let mut safe_reports: usize = 0;
    for line in reader.lines() {
        let line = line.expect("failed to read line");
        let report: Vec<i32> = line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect();
        let trend: bool = find_trend(&report);
        safe_reports += test_report(&report, trend, false);
    }
    println!("{safe_reports}");
}

//if increasing true else false
fn find_trend(report: &Vec<i32>) -> bool {
    report[0] < report[1]
}

pub fn test_report(report: &Vec<i32>, trend: bool, second_iter: bool) -> usize {
    let mut p1: usize = 0;
    let mut p2: usize = 1;
    while p2 < report.len() {
        let difference: i32;
        if trend {
            difference = report[p2] - report[p1]
        } else {
            difference = report[p1] - report[p2];
        }
        if difference <= 0 || difference > 3 {
            if second_iter {
                return 0;
            }
            return damper_check(&report, &p1, &p2);
        }
        p1 += 1;
        p2 += 1;
    }
    1
}
fn damper_check(report: &Vec<i32>, _p1: &usize, p2: &usize) -> usize {
    for index in 0..*p2 + 1 {
        let mut original_copy: Vec<i32> = report.clone();
        original_copy.remove(index);
        let trend = find_trend(&original_copy);
        let sample_check = test_report(&original_copy, trend, true);
        if sample_check == 1 {
            return 1;
        }
    }
    0
}
