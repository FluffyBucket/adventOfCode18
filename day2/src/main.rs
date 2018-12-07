use std::fs;
use std::collections::HashMap;
use std::time::{Duration, Instant};


fn main() {
    let start = Instant::now();
    let mut x: i64 = 0;
    for _ in 0..100 {
        x = get_checksum();
    };

    let end = start.elapsed().subsec_nanos();
    println!("Value: {} \tTime: {}ns",x, end/100);
}

fn get_checksum() -> i64 {
    //let start = Instant::now();
    let content = fs::read_to_string("input.txt").unwrap();
    //let mut vec_chars: Vec<char>;
    let mut two_times_count = 0;
    let mut three_times_count = 0;

    for line in content.lines() {
        let vec_chars: Vec<char> = line.chars().collect();
        //vec_chars.sort_by(|a, b| b.cmp(a));
        let mut char_count = HashMap::new();
        let mut two_times = 0; 
        let mut three_times = 0;
        for c in vec_chars {
            let count = char_count.entry(c).or_insert(0);
            *count += 1;
        };

        for (k, v) in char_count.iter() {
            if *v == 2 {
                two_times = 1;
            } else if *v == 3 {
                three_times = 1;
            }
        }

        two_times_count += two_times;
        three_times_count += three_times;
    };

    two_times_count * three_times_count
    
    //let end = start.elapsed().subsec_nanos();
    //println!("Checksum: {}\tTime: {}ns",checksum,end);
}