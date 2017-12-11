use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*; //brings in lines on file
use std::str;

pub fn run() {
    let file = "input/input4.txt";
    let file = File::open(file).expect(&format!("File not found {}", file));
    let file = BufReader::new(&file);

    let mut count = 0;
    let mut not_anagram_count = 0;
    for (_, l) in file.lines().enumerate() {
        let mut words = l.unwrap().split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
        words.sort();
        let mut dup_found = false;
        for i in 0..words.len() - 1 {
            if words[i] == words[i+1] {
                dup_found = true;
                break;
            }
        }
        if !dup_found {
            count += 1;
        }
        let mut ordered_words = words.iter().map( |s| {
            let mut x = s.bytes().collect::<Vec<u8>>();
            x.sort();
            let res = str::from_utf8(&x);
            res.unwrap().to_string()
        }).collect::<Vec<String>>();
        ordered_words.sort();
        
        dup_found = false;

        for i in 0..ordered_words.len() - 1 {
            if ordered_words[i] == ordered_words[i+1] {
                dup_found = true;
                break;
            }
        }
        if !dup_found {
            not_anagram_count += 1;
        }

    }
    println!("No Duplicates: {}", count);
    println!("No Anagrams: {}", not_anagram_count);
}