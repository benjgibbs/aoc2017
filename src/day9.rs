
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


pub fn run() {
    let file = "input/input9.txt";
    let file = File::open(file).expect(&format!("File not found {}", file));
    let file = BufReader::new(&file);

    for (_, line) in file.lines().enumerate() {
        let line: String = line.unwrap();
        let mut group_depth = 0;
        let mut score = 0;
        let mut skip = false;
        let mut in_garbage = false;
        let mut garbage_count = 0;
        for c in line.chars() {
            if skip {
                skip = false;
                continue;
            }
            match c {
                '{' => {
                    if in_garbage {
                        garbage_count += 1;
                    } else {
                        group_depth += 1;
                    }
                }
                '}' => {
                    if in_garbage {
                        garbage_count += 1;
                    } else {
                        score += group_depth;
                        group_depth -= 1;
                    }
                }
                '<' => {
                    if in_garbage {
                        garbage_count += 1;
                    } else {
                        in_garbage = true;
                    }
                }
                '>' => in_garbage = false,
                '!' => skip = true,
                _ => {
                    if in_garbage {
                        garbage_count += 1;
                    }
                }

            }
        }
        println!("Score: {}", score);
        println!("Garbage Count: {}", garbage_count);
    }
}
