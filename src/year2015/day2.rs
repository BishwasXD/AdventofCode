use std::fs::File;
use std::io::{BufRead, BufReader};
pub fn no_math()
{
    println!("from day 2");
    let file = File::open("src/year2015/input2.txt").expect("couldnt open the file");
    let reader = BufReader::new(file);
    let mut order_area: u32 = 0;
    for line in reader.lines(){
        let line = line.expect("couldnt read the line");
        order_area += parse_dimensions(&line);
    }
    println!("Final area : {order_area}");
    
     
}
pub fn parse_dimensions(line: &str) -> u32{

 let dimensions: Vec<&str> = line.split("x").collect();
 let mut parsed_dimensions: Vec<u32> = Vec::new();
 for item in dimensions{
 let parsed: u32 = item.parse().expect("couldnt parse");
 parsed_dimensions.push(parsed)
 }
 parsed_dimensions.sort();
 //let wrapping_area: u32 = parsed_dimensions[0]*parsed_dimensions[1]+ 2*parsed_dimensions[0]*parsed_dimensions[1] + 2*parsed_dimensions[1]*parsed_dimensions[2] + 2*parsed_dimensions[0]*parsed_dimensions[2];
 //return wrapping_area;
 let ribbon_length: u32 = 2*parsed_dimensions[0] + 2*parsed_dimensions[1];
 let bow_length: u32 = parsed_dimensions[0]*parsed_dimensions[1]*parsed_dimensions[2];
 let total: u32 = ribbon_length + bow_length;
 return total;
}
