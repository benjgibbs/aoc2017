
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;
use std::str::FromStr;


pub fn run() {
    let file = "input/input8.txt";
    let file = File::open(file).expect(&format!("File not found {}", file));
    let file = BufReader::new(&file);

    let mut regs: HashMap<String, i64> = HashMap::new();
    let mut at_max: i64 = 0;
    for (_, line) in file.lines().enumerate() {
        let parts: Vec<String> = line.unwrap()
                                     .split(" ")
                                     .map(|s| s.to_string())
                                     .collect();
        let r1: &str = &parts[0];
        let op: &str = &parts[1];
        let v1: i64 = i64::from_str(&parts[2]).unwrap();
        let r2: &str = &parts[4];
        let cond: &str = &parts[5];
        let v2: i64 = i64::from_str(&parts[6]).unwrap();
        let r1v: i64 = *regs.get(r1).unwrap_or(&0);
        let r2v: i64 = *regs.get(r2).unwrap_or(&0);
        let new_val = if op == "inc" {
            r1v + v1
        } else {
            r1v - v1
        };
        match cond {
            "<=" => {
                if r2v <= v2 {
                    regs.insert(r1.to_string(), new_val);
                    if new_val > at_max {
                        at_max = new_val
                    }
                }
            }
            "<" => {
                if r2v < v2 {
                    regs.insert(r1.to_string(), new_val);
                    if new_val > at_max {
                        at_max = new_val
                    }
                }
            }
            ">" => {
                if r2v > v2 {
                    regs.insert(r1.to_string(), new_val);
                    if new_val > at_max {
                        at_max = new_val
                    }
                }
            }
            ">=" => {
                if r2v >= v2 {
                    regs.insert(r1.to_string(), new_val);
                    if new_val > at_max {
                        at_max = new_val
                    }
                }
            }
            "==" => {
                if r2v == v2 {
                    regs.insert(r1.to_string(), new_val);
                    if new_val > at_max {
                        at_max = new_val
                    }
                }
            }
            "!=" => {
                if r2v != v2 {
                    regs.insert(r1.to_string(), new_val);
                    if new_val > at_max {
                        at_max = new_val
                    }
                }
            }
            &_ => {
                panic!("unknown: {}", cond);
            }
        }
    }
    let mut max: i64 = 0;
    for (_, v) in regs {
        if v > max {
            max = v;
        }
    }
    println!("Max: {}. All time: {}", max, at_max);
}
