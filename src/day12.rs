
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::str::FromStr;

pub fn run() {
    let file = "input/input12.txt";
    let file = File::open(file).expect(&format!("File not found {}", file));
    let file = BufReader::new(&file);

    let mut parents: Vec<usize> = Vec::new();
    let mut counts: Vec<u32> = Vec::new();
    for i in 0..2000{
        parents.push(i);
        counts.push(1);
    }

    for (_, line) in file.lines().enumerate() {
        let line: String = line.unwrap();
        let parts: Vec<&str> = line.split(" <-> ").collect();
        let from: usize = usize::from_str(parts[0]).unwrap();
        let tos: Vec<usize> = parts[1]
                               .split(", ")
                               .map(|s| usize::from_str(s).unwrap())
                               .collect();
        
        for to in tos {
            let from_parent = find_parent(&parents, from);
            let from_count = counts[from_parent];
            let to_parent = find_parent(&parents, to);
            let to_count = counts[to_parent];
            let new_count = from_count + to_count;
            //println!("from={}, to={}, from_parent={}, to_parent={}, from_count={}, to_count={}", 
            //    from, to, from_parent, to_parent, from_count, to_count);
            if from_parent == to_parent {
                continue;
            }
            if from_count < to_count {
                parents[from_parent] = to_parent;
                counts[to_parent] = new_count;
                counts[from_parent] = 0;
            } else {
                parents[to_parent] = from_parent;
                counts[from_parent] = new_count;
                counts[to_parent] = 0;
            }
        }

    }
    let count = counts[find_parent(&parents, 0)];
    println!("In group 0: {}", count);

    let unique_parents = counts.iter().filter(|c| **c != 0 ).count();
    
    println!("Num groups: {}", unique_parents);

}

fn find_parent(parents: &Vec<usize>, mut find: usize) -> usize {
    while find != parents[find] {
        //println!("find={}, parents[find]={}", find, parents[find]);
        find = parents[find];
    }
    find
}
