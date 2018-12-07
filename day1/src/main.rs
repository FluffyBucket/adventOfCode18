use std::fs;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};
use std::collections::HashMap;

fn main() {
    get_first_instance();
    //get_sum();
}

fn get_sum() {
    let start = Instant::now();
    let mut sum: i64 = 0;
    
    let file = fs::File::open("input.txt").expect("Something went wrong reading file!");
    // Reading file line by line
    for line in BufReader::new(file).lines() {
        // let l = match line {
        //     Ok(string) => string,
        //     Err(_error) => {
        //         panic!("Something went wrong when reading a line");
        //     },
        // };
        
        let num = line.unwrap().parse().unwrap_or(0);
        //println!("Input {}\t Output {}",l,num);
        
        sum += num;
    };
    
    let end = start.elapsed().subsec_nanos();
    
    println!("Value: {}\tElapsed time: {}ns",sum,end);
}

fn get_first_instance() {
    let start = Instant::now();
    let mut sum: i64 = 0;
    let mut repeats = HashMap::new();
    
    repeats.insert(0 as i64,0);
    //let file = fs::File::open("input.txt").expect("Something went wrong reading file!");
    // Reading file line by line
    let content = fs::read_to_string("input.txt").unwrap();

    'outer:loop {

        for line in content.lines() {
            // let l = match line {
            //     Ok(string) => string,
            //     Err(_error) => {
            //         panic!("Something went wrong when reading a line");
            //     },
            // };
            
            let num = line.parse().unwrap_or(0);
            //println!("Input {}\t Output {}",l,num);
            
            sum += num;
            if repeats.contains_key(&sum) {
                let end = start.elapsed().subsec_nanos();
                println!("First repeat: {}\tTime: {}ns",sum,end);
                break 'outer;
            } else {
                repeats.insert(sum,0);
            }
        };
    };
}