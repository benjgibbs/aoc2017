

use std::str::FromStr;
use std::iter::Iterator;
use std::vec::Vec;

pub fn run() {
    let input = "4	1	15	12	0	9	9	5	5	8	7	3	14	5	12	3";
    let mut input = input.split("\t").map(|i| i32::from_str(&i).unwrap()).collect::<Vec<i32>>();
    println!("Input: {:?}", input);
    let mut history: Vec<Vec<i32>> = Vec::new();
    
    let mut cont = true;
    let mut count = 0;
    let len = input.len() as i32;
    while cont {
        
        match history.iter().find(|&ref i| **i == input) {
            Some(_) => {
                cont = false;
            },
            None => {
                count += 1;
                history.push(input.clone());
                
                let max_val : i32 = *input.iter().max().unwrap();
                let max_idx : usize = input.iter().position(|x| *x == max_val).unwrap();
                input[max_idx] = 0;
                let add_to_all = max_val / len;
                let one_more_for = (max_val as usize) % input.len();

                let mut i = 0;
                let mut j = (max_idx + 1) % input.len();
                while i < input.len() {
                    let a = if i < one_more_for {1} else {0};
                    input[j] += add_to_all + a;
                    j = (j + 1) % input.len();
                    i += 1;
                }
            }
        }
    }
    println!("{:?}", count);

    let first = history.iter().position(|&ref i| *i == input).unwrap() as i32;
    println!("{:?}", count - first);
}