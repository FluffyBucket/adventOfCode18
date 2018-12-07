use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut sum: i64 = 0; 
    
    let file = File::open("input.txt").expect("Something went wrong reading file!");
    // Reading file line by line
    for line in BufReader::new(file).lines() {
        let l = match line {
            Ok(string) => string,
            Err(_error) => {
                panic!("Something went wrong when reading a line");
            },
        };
        
        let num = l.parse().unwrap_or(0);
        //println!("Input {}\t Output {}",l,num);
        
        sum += num;
    };
    println!("Value: {}",sum);
}