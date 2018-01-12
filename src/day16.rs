use std::fs::File;  
use std::io::BufReader;
use std::io::prelude::*;

use std::char;
use std::str::FromStr;

pub fn run() {
    let file = "input/input16.txt";
    let file = File::open(file).expect(&format!("File not found {}", file));
    let mut file = BufReader::new(&file);
    
    let mut input = String::new(); 
    let progs : Vec<char> = (0..16)
                        .map(|u| char::from_u32(('a' as u32) + u).unwrap())
                        .collect();

    file.read_to_string(&mut input).expect("Failed to read file");
    perfom_dance(&input, &mut progs.clone(), 1);
    perfom_dance(&input, &mut progs.clone(), 1_000_000_000);
}

fn swap(mv: &str) -> Box<Fn(&mut Vec<char>) -> ()> {
    let sp : usize = usize::from_str(&mv[1..]).unwrap();
    Box::new(move |progs: &mut Vec<char>| {
        let mut buf : Vec<char> = vec![' '; progs.len()];
        for i in 0..sp {
            buf[i] = progs[progs.len() - sp + i];
        }
        for i in sp..progs.len() {
            buf[i] = progs[i-sp];
        }
        for i in 0..progs.len() {
            progs[i] = buf[i];
        }
    })
}

fn exch(mv: &str) ->  Box<Fn(&mut Vec<char>) -> ()> {
    let xs : Vec<usize> =  mv[1..]
                        .split("/")
                        .map(|x| usize::from_str(x).unwrap())
                        .collect();
    Box::new(move |progs: &mut Vec<char>| {
        let t = progs[xs[0]];
        progs[xs[0]] = progs[xs[1]];
        progs[xs[1]] = t;
    })                       
}

fn partner(mv: &str) ->  Box<Fn(&mut Vec<char>) -> ()> {

    let pa : char = mv.chars().nth(1).unwrap();
    let pb : char = mv.chars().nth(3).unwrap();
    Box::new(move |progs: &mut Vec<char>| {
        for i in 0..progs.len() {
            if progs[i] == pa {
                progs[i] = pb;
            } else if progs[i] == pb {
                progs[i] = pa;
            }
        }
    })
}

fn perfom_dance(input: &String, progs: &mut Vec<char>, iter: u32) {

    let mut funcs : Vec<Box<Fn(&mut Vec<char>) -> ()>>  = Vec::new();
    for (_,mv) in input.split(",").enumerate() {
        match mv.chars().nth(0).unwrap() {
            's' => funcs.push(swap(mv)),
            'x' => funcs.push(exch(mv)),
            'p' => funcs.push(partner(mv)),
            _ => panic!(format!("unknown move {}", mv))
        }
    }
    

    let start : Vec<char> = progs.clone();
    let mut cycle_check : Vec<char> = progs.clone();

    let mut cycle_len = 0;
    loop {
        cycle_len += 1;
        for f in &funcs {
            f(&mut cycle_check);
        }
        if cycle_check == start {
            break;
        }
    }
    println!("cycle_len: {}", cycle_len);
    for _ in 0..(iter % cycle_len) {
        for f in &funcs {
            f(progs);
        }
        let s : String = progs.clone().into_iter().collect();
            println!("\t{}", s);
    }

    let s : String = progs.clone().into_iter().collect();
    println!("{}", s);
}