use std::fs::File;
use std::collections::HashSet;
use std::io::BufReader;
use std::io::prelude::*; //brings in lines on file
use regex::Regex;
use regex::Captures;
use std::str::FromStr;

use std::collections::HashMap;

pub fn run() {
    let file = "input/input7.txt";
    let file = File::open(file).expect(&format!("File not found {}", file));
    let file = BufReader::new(&file);

    let mut weights: HashMap<String, i32> = HashMap::new();
    let mut edges: HashMap<String, Vec<String>> = HashMap::new();
    let mut destination_set: HashSet<String> = HashSet::new();
    let mut source_set: HashSet<String> = HashSet::new();
    let re = Regex::new(r"(\w+) \((\d+)\)(?: -> ((?:\w+(?:, )?)+))?").unwrap();
    for (_, l) in file.lines().enumerate() {
        let line = l.unwrap();
        let cap: Captures = re.captures(&line).unwrap();
        let from: String = String::from(cap.get(1).unwrap().as_str());

        source_set.insert(from.clone());

        let weight: i32 = i32::from_str(cap.get(2).unwrap().as_str()).unwrap();
        weights.insert(from.clone(), weight);

        let list: &str = cap.get(3).map_or("", |m| m.as_str());
        let mut these_edges: Vec<String> = Vec::new();
        for mut end in list.split(", ") {
            destination_set.insert(String::from(end));
            these_edges.push(String::from(end));
        }
        edges.insert(String::from(from), these_edges);
    }

    let mut find_root: HashSet<String> = source_set.clone();
    for k in destination_set {
        find_root.remove(&k);
    }
    let root: &String = find_root.iter().next().unwrap();
    println!("Root: {:?}", root);
    calc_weights(root, &edges, &weights);
}

fn calc_weights(
    root: &String,
    edges: &HashMap<String, Vec<String>>,
    weights: &HashMap<String, i32>,
) -> i32 {
    //println!("\n** {}: {:?}",root, weights );
    let mut total_weight: i32 = *weights.get(root).unwrap();

    match edges.get(root) {
        Some(children) => {
            let mut child_weights: HashMap<String, i32> = HashMap::new();
            let mut distinct_child_weights: HashSet<i32> = HashSet::new();
            for child in children {
                if child.len() > 0 {
                    let child_weight = calc_weights(child, edges, weights);
                    child_weights.insert(child.clone(), child_weight);
                    total_weight += child_weight;
                    distinct_child_weights.insert(child_weight);
                }
            }
            if child_weights.len() > 1 && distinct_child_weights.len() > 1 {
                println!("{:?}", child_weights);
                for child in children {
                    println!("\t{}: {}", child, weights.get(child).unwrap());
                }
            }
        }, 
        None => {}
    };
    total_weight
}
