use md5;

pub fn  coin_mining(){
 println!("day 4");
 let secret_key = "ckczppom";
 let mut n: usize = 0;
 loop{
 let input = secret_key.to_owned() + &(n).to_string();
 let digest = format!("{:x}",md5::compute(input));
// println!("{digest}");
 if digest.starts_with("000000"){
     println!("solution is : {}", n);
     break;
 }
n = n + 1;
 
 }

} //probably brute force the only way, unless parallel processing but solution remains the same. 
