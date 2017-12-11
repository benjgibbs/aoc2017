use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*; //brings in lines on file
use std::str::FromStr;

pub fn run() {
    let file = "input/input5.txt";
    let file = File::open(file).expect(&format!("File not found {}", file));
    let file = BufReader::new(&file);
    let mut memory : Vec<i32> = Vec::new();
    for (_, l) in file.lines().enumerate() {
        let line = l.unwrap();
        let num = i32::from_str(&line).unwrap();
        memory.push(num)
    }
    part1(memory.clone());
    part2(memory);
}

fn part1(mut memory: Vec<i32>) {
    let mut pos : i32 = 0;
    let mut count : u32 = 0;
    while pos >= 0 && (pos as usize) < memory.len() {
        let jmp = memory[(pos as usize)];
        memory[pos as usize] += 1;
        pos += jmp;
        count += 1;
    }
    println!("Steps: {}", count);

}

fn part2(mut memory: Vec<i32>) {
    let mut pos : i32 = 0;
    let mut count : u32 = 0;
    while pos >= 0 && (pos as usize) < memory.len() {
        let jmp = memory[(pos as usize)];
        if jmp > 2 {
            memory[pos as usize] -= 1;
        } else {
            memory[pos as usize] += 1;
        }
        pos += jmp;
        count += 1;
    }
    println!("Steps: {}", count);
}