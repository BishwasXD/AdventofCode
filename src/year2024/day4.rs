use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn ceres_search(){
    println!("day 4");
    let file = File::open("src/year2024/input4.txt").expect("failed to open the file");
    let mut puzzle: Vec<Vec<char>> = Vec::new();
    let reader = BufReader::new(file);
    for line in reader.lines(){
        let line = line.expect("err reading line").chars().collect();
        puzzle.push(line);

    }
    
 //create a window of length 4 and searching, but inefficent, lets track X first p1 is at X and
 //make a string by combining next 3 chars check if it is xmas yes count increase, this to be done
 //while p1 < rowlength only horizontal and reverse horizontal  2 cases 
 //(i,j) i is same j increases
 //for vertical search stay in same row and increase column we go down this way reverse the text
 //for reverse searching which goes upward do this while c < columnLength
 //(i,j) j is same i increases
 //for diagonal search row position remains the same column position increases by 1 each time 
//(i,j) both increases (i+1, j+1) can start diagonal from 4 different positions 
    let mut occurances: usize = 0;
    let mut valid_occurences: usize = 0;
    let is_second_part = true;
    let  check_string = String::from("MAS");

    if !is_second_part{
        for (i,_) in puzzle.iter().enumerate(){
            for (j, _) in puzzle[i].iter().enumerate(){
                if puzzle[i][j] == 'X'{
    if j > 2 {
        let rev_hor = format!("{}{}{}", puzzle[i][j-1], puzzle[i][j-2], puzzle[i][j-3]);
        if rev_hor == check_string {
        occurances += 1;
    }
}
    if j < puzzle[i].len() - 3 {
        let hor = format!("{}{}{}", puzzle[i][j+1], puzzle[i][j+2], puzzle[i][j+3]);
        if hor == check_string {
        occurances += 1;
    }
}

// Search vertically
    if i > 2 {
        let rev_ver = format!("{}{}{}", puzzle[i-1][j], puzzle[i-2][j], puzzle[i-3][j]);
        if rev_ver == check_string {
        occurances += 1;
    }
}
    if i < puzzle.len() - 3 {
        let ver = format!("{}{}{}", puzzle[i+1][j], puzzle[i+2][j], puzzle[i+3][j]);
        if ver == check_string {
        occurances += 1;
    }
}

// Search diagonally
    if i < puzzle.len() - 3 && j < puzzle[i].len() - 3 {
        let d_right = format!("{}{}{}", puzzle[i+1][j+1], puzzle[i+2][j+2], puzzle[i+3][j+3]);
        if d_right == check_string {
        occurances += 1;
    }
}
    if i < puzzle.len() - 3 && j > 2 {
        let d_left = format!("{}{}{}", puzzle[i+1][j-1], puzzle[i+2][j-2], puzzle[i+3][j-3]);
        if d_left == check_string {
        occurances += 1;
    }
}
    if i > 2 && j < puzzle[i].len() - 3 {
        let du_right = format!("{}{}{}", puzzle[i-1][j+1], puzzle[i-2][j+2], puzzle[i-3][j+3]);
        if du_right == check_string {
        occurances += 1;
    }
}
    if i > 2 && j > 2 {
        let du_left = format!("{}{}{}", puzzle[i-1][j-1], puzzle[i-2][j-2], puzzle[i-3][j-3]);
        if du_left == check_string {
        occurances += 1;
    }
}
           }
        }

    }
    }
    else{
for i in 0..puzzle.len() {
    for j in 0..puzzle[i].len() {
        if puzzle[i][j] == 'A' {
            if i > 0 && i < puzzle.len() - 1 && j > 0 && j < puzzle[i].len() - 1 {
                let right = format!("{}{}{}", puzzle[i-1][j-1], puzzle[i][j], puzzle[i+1][j+1]);
                let left = format!("{}{}{}", puzzle[i+1][j-1], puzzle[i][j], puzzle[i-1][j+1]);
                if (left == "MAS" || left == "SAM") && (right == "MAS" || right == "SAM"){
                    valid_occurences += 1
                }
            }
        }
    }

}    }
    
    println!("{valid_occurences}");
    println!("{occurances}")
}


