use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::cmp::max;
use std::cmp::min;
use regex::Regex;

pub fn run() {
    let file = "input/input2.txt";
    let file = File::open(file).expect(&format!("File not found {}", file));
    let file = BufReader::new(&file);

    let mut count = 0;
    let mut count2 = 0;

    let re = Regex::new(r"\s+").unwrap();
    for (_, line) in file.lines().enumerate() {
        let line = line.unwrap();
        
        let nums = re.split(&line);
        let nums = nums.map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>();

        for i in 0..nums.len() - 1 {
            for j in i+1..nums.len() {
                let m1 = max(nums[i], nums[j]);
                let m2 = min(nums[i], nums[j]);
                
                if m1 % m2 == 0 {
                    count2 += m1/m2;
                    break;
                }
            }
        }

        let m1 = nums.iter().max().unwrap();
        let m2 = nums.iter().min().unwrap();
        count += m1 - m2;
    }
    println!("{}", count);
    println!("{}", count2);

}