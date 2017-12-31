

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::str::FromStr;
use std::collections::HashMap;

pub fn run() {
    let file = "input/input13.txt";
    let file = File::open(file).expect(&format!("File not found {}", file));
    let file = BufReader::new(&file);
    let mut map : HashMap<usize, usize> = HashMap::new();
    for (_, line) in file.lines().enumerate() {
        let parts :Vec<usize> = line.unwrap()
                                    .split(": ")
                                    .map(|s| usize::from_str(s).unwrap())
                                    .collect();
        map.insert(parts[0], parts[1]);
    }
    let mut score : usize = 0;
    for (depth,range) in &mut map {
        if hit(*depth,*range) {
            score += *depth * *range;
        }
    }
    println!("Score: {}", score);
    
    let mut delay = 4;
    while caught(delay, &mut map) {
        delay += 1;
    }
    println!("Delay: {}", delay);

}

fn caught(delay: usize, map: &HashMap<usize, usize>) -> bool {
    for (depth, range) in map {
        if hit(delay + depth, *range) {
            return true;
        }
    }
    return false;
}

fn hit(time: usize, range: usize) -> bool{
    time % (2 * (range - 1)) == 0
}