use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::str::FromStr;
use std::collections::HashSet;

#[derive(Debug,Copy)]
struct Particle {
    p: (i64, i64, i64),
    v: (i64, i64, i64),
    a: (i64, i64, i64),
}

impl Clone for Particle {
    fn clone(self: &Particle) -> Particle {
        Particle{p:self.p, v: self.v, a: self.a}
    }
}


impl Particle {
    fn new(line: &str) -> Particle {
        let line = line.trim();
        let parts : Vec<&str> = line[..line.len() - 1].split(">, ").map(|s| &s[3..]).collect();
        let xs : Vec<(i64, i64, i64)> = parts.iter().map(|s| {
                let xs :Vec<i64> = s.split(",").map(|t| i64::from_str(t).unwrap()).collect();
                (xs[0], xs[1], xs[2])
            }).collect();
            
        Particle{p:xs[0],v:xs[1],a:xs[2]}
    }

    fn total_acceleration(self: &Particle) -> i64 {
        let (a1, a2, a3) = self.a;
        a1*a1 + a2*a2 + a3*a3
    }

    fn next_pos(self: &Particle) -> Particle {
        
        let v2 = (self.v.0 + self.a.0, self.v.1 + self.a.1, self.v.2 + self.a.2);
        let p2 = (self.p.0 + v2.0, self.p.1 + v2.1, self.p.2 + v2.2);
        
        Particle{p:p2, v: v2, a: self.a}
    }

    fn collides_with(self: &Particle, this: &Particle) -> bool {
        self.p.0 == this.p.0 && self.p.1 == this.p.1 && self.p.2 == this.p.2
    }
}


pub fn run() {
    let file = "input/input20.txt";
    let file = File::open(file).expect(&format!("File not found {}", file));
    let file = BufReader::new(&file);
    let particles : Vec<Particle> = file.lines().map(|l| Particle::new(&l.unwrap())).collect();

    part1(&particles);
    part2(&particles);
}

fn collisions(particles: &Vec<Particle>) -> HashSet<usize> {
    let mut result : HashSet<usize> = HashSet::new();
    for i in 0..particles.len()-1 {
        for j in i+1..particles.len() {
            if particles[i].collides_with(&particles[j]) {
                result.insert(i);
                result.insert(j);
            }
        }
    }
    result
}

fn remove_collisions(particles: &Vec<Particle>, collisions: HashSet<usize>) -> Vec<Particle> {
    
    let mut result : Vec<Particle> = Vec::new();

    for i in 0..particles.len() {
        if !collisions.contains(&i) {
            result.push(particles[i]);
        }
    }
    result
}

fn part2(particles: &Vec<Particle>){
    let mut particles = particles.clone();
    let mut len = usize::max_value();
    let mut iter = 0;
    while particles.len() > 0 {
        iter += 1;
        
        let collisions = collisions(&particles);
        if collisions.len() > 0 {
            println!("Collisions: {:?}", collisions);
            particles = remove_collisions(&particles, collisions);
        }
        particles = particles.iter().map(|p| p.next_pos()).collect();
        //println!("particles[0]={:?}", particles[0]);

        if particles.len() < len {
            println!("paricles.len()={}, iter={}", particles.len(), iter);
            //println!("{:?}", particles);
            len = particles.len();
        }
    }
}

fn part1(particles: &Vec<Particle>){
    let mut min_acc = i64::max_value();
    let mut min_pos = 0;
    for i in 0..particles.len() {
        let acc = particles[i].total_acceleration();
        if acc < min_acc {
            min_acc = acc;
            min_pos = i;
        }
    }
    println!("Min Acc: {}", min_pos);
}