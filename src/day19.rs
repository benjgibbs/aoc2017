use std::fs::File;  
use std::io::BufReader;
use std::io::prelude::BufRead;


#[derive(Debug,PartialEq)]
enum Dir {
    UP,
    RIGHT,
    DOWN,
    LEFT
}

fn naviagate(grid : &Vec<&[u8]>, dir: Dir, pos: (i32, i32)) -> bool {
    let (mut x, mut y) = pos;
    
    loop {
        //println!("Position: ({},{}) Direction: {:?}", x, y, dir);
        if x < 0 || y < 0 || y >= grid.len() as i32 || x > grid[y as usize].len() as i32 {
            return false
        }
        let c = grid[y as usize][x as usize] as char;
        //println!("Char: {}", c); 
        match  c {
            ' ' => {
                return false;
            },
            '+' => {
                if dir != Dir::DOWN && naviagate(grid, Dir::UP, (x, y-1)) {
                    return true
                }
                if dir != Dir::LEFT && naviagate(grid, Dir::RIGHT, (x+1, y)) {
                    return true
                }
                if dir != Dir::UP && naviagate(grid, Dir::DOWN, (x, y+1)) {
                    return true
                }
                if dir != Dir::RIGHT && naviagate(grid, Dir::LEFT, (x-1, y)) {
                    return true
                }
                return false;
            },
            c => {
                if !(c== '|' ||  c == '-' ){
                    print!("{}", c);
                }
                match dir {
                    Dir::UP => y -= 1,
                    Dir::RIGHT => x += 1,
                    Dir::DOWN => y += 1,
                    Dir::LEFT => x -= 1,
                }
            },
        }
    }
}

pub fn run() {
    let file = "input/input19.txt";
    let file = File::open(file).expect(&format!("File not found {}", file));
    let file = BufReader::new(&file);
    let lines : Vec<String> = file.lines().map(|l| l.unwrap()).collect();
    let grid : Vec<&[u8]> = lines.iter().map(|l| l.as_bytes()).collect();
    let mut pos : (i32, i32) = (0,0);
    
    for i in 0..grid[0].len() {
        if grid[0][i] == '|' as u8 {
            pos = (i as i32, 0);
            break;
        }
    }
    println!("Start: {:?} {:?}", pos, Dir::DOWN);
    naviagate(&grid, Dir::DOWN, pos);
    println!();
}