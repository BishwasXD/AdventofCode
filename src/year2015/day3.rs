use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
pub fn present_delivery(){
    println!("day 3");
    let mut file = File::open("src/year2015/input3.txt").expect("couldnt open the file");
    let mut directions = String::new();
    file.read_to_string(&mut directions).expect("couldnt read a file");
    let is_robo: bool = true;
    if is_robo{
        let mut robo_div = String::new();
        let mut santa_div = String::new();
        for (index, dir) in directions.chars().enumerate(){
            if index % 2 == 0{
                santa_div.push(dir);
            }
            else{
                robo_div.push(dir);
            }
        }
       let robo_visit = find_visited(robo_div);
       let santa_visit = find_visited(santa_div);
       let union: HashSet<(i32, i32)>  = robo_visit.union(&santa_visit).cloned().collect();
       let houses_visited = union.len();
       println!("{houses_visited}");
    }
    else{
        let total_set = find_visited(directions);
        let houses_visited = total_set.len();
        println!("{houses_visited}");
    }
}

pub fn find_visited(directions: String) -> HashSet<(i32, i32)>{

  let mut x:i32 = 0;
  let mut y:i32 = 0;
  let mut coordinates_set: HashSet<(i32, i32)> = HashSet::new();
  coordinates_set.insert((x,y));
  for char in directions.chars(){
       match char {
           '^' => y += 1,
           'v' => y -= 1,
           '>' => x += 1,
           '<' => x -= 1,
           _ => println!("do nothing")
       }

      coordinates_set.insert((x,y));


  }
  return coordinates_set;
}

