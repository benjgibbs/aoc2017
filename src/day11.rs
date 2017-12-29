

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


pub fn run() {

    let file = "input/input11.txt";
    let file = File::open(file).expect(&format!("File not found {}", file));
    let file = BufReader::new(&file);

    for (_, line) in file.lines().enumerate() {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut z: i32 = 0;
        let mut max: i32 = 0;
        for (_, s) in line.unwrap().split(",").enumerate() {
            match s {
                "n" => {
                    y += 1;
                    z -= 1;
                }
                "ne" => {
                    x += 1;
                    z -= 1;
                }
                "se" => {
                    x += 1;
                    y -= 1;
                }
                "s" => {
                    y -= 1;
                    z += 1;
                }
                "sw" => {
                    x -= 1;
                    z += 1;
                }
                "nw" => {
                    x -= 1;
                    y += 1;
                }
                &_ => panic!("Unknown direction: {}", s),
            }
            let d = distance(x, y, z);
            if d > max {
                max = d;
            }

        }
        let distance = distance(x, y, z);
        println!("distance: {}", distance);
        println!("max: {}", max);

    }
}

fn distance(x: i32, y: i32, z: i32) -> i32 {
    (x.abs() + y.abs() + z.abs()) / 2
}
