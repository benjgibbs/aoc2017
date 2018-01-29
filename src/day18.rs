use std::fs::File;  
use std::io::BufReader;
use std::io::prelude::*;

use std::time::Duration;


use std::str::FromStr;
use std::collections::HashMap;

use std::thread;
use std::sync::Arc;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;


struct Vm {
    pc: usize,
    reg: HashMap<char,i64>,
    last_snd: i64,
    send_count: i32
}

impl Vm {
    
    fn new(p: i64) -> Vm {
        let mut res = Vm {
            pc: 0,
            reg: HashMap::new(),
            last_snd: 0,
            send_count: 0,
        };
        for r in 0..26 {
            let c : char = (('a' as u8) + r) as char;
            res.reg.insert(c, 0);
        }
        res.reg.insert('p', p);        
        res
    }
    fn deref_string(self: &Vm, s: &str) -> i64 {
        let first_char : char = s.as_bytes()[0] as char;
        if s.len() == 1 && first_char as u8  >= 'a' as u8 && first_char as u8 <= 'z' as u8 {
            *self.reg.get(&first_char).unwrap()
        } else {
            i64::from_str(s).unwrap()
        }
    }

    fn run(mut self: &mut Vm, prog: &Vec<Box<Fn(&mut Vm) -> usize>>) {
        while self.pc < prog.len()  {
            let fun :& Box<Fn(&mut Vm) ->  usize> = prog.get(self.pc).unwrap();
            let next_pc = fun(&mut self);
            self.pc = next_pc;
        }
    }
}

fn snd(line: &str) -> Box<Fn(&mut Vm) ->  usize> {
    let parts : Vec<String> = line.split(" ").map(|x| String::from(x)).collect();
    Box::new(move |vm: &mut Vm| {
        vm.last_snd = vm.deref_string(&parts[1]);
        vm.pc + 1
    })
}

fn rcv(line: &str) -> Box<Fn(&mut Vm) ->  usize> {
    let parts : Vec<String> = line.split(" ").map(|x| String::from(x)).collect();
    Box::new(move |vm: &mut Vm| {
        if vm.deref_string(&parts[1]) != 0 {
            println!("Recovered: {}hz", vm.last_snd);
            usize::max_value()
        } else {
            vm.pc + 1
        }
    })
}

fn set(line: &str) -> Box<Fn(&mut Vm) ->  usize> {
    let parts : Vec<String> = line.split(" ").map(|x| String::from(x)).collect();
    Box::new(move |vm: &mut Vm| {
        let reg = parts[1].as_bytes()[0] as char;
        let val = vm.deref_string(&parts[2]);
        vm.reg.insert(reg, val);
        vm.pc + 1
    })
}

fn mul(line: &str) -> Box<Fn(&mut Vm) ->  usize> {
    let parts : Vec<String> = line.split(" ").map(|x| String::from(x)).collect();
    Box::new(move |vm: &mut Vm| {
        let reg = parts[1].as_bytes()[0] as char;
        let xval = *vm.reg.get(&reg).unwrap();
        let yval = vm.deref_string(&parts[2]);
        vm.reg.insert(reg, xval * yval);
        vm.pc + 1
    })
}

fn add(line: &str) -> Box<Fn(&mut Vm) ->  usize> {
    let parts : Vec<String> = line.split(" ").map(|x| String::from(x)).collect();
    Box::new(move |vm: &mut Vm| {
        let reg = parts[1].as_bytes()[0] as char;
        let xval = *vm.reg.get(&reg).unwrap();
        let yval = vm.deref_string(&parts[2]);
        vm.reg.insert(reg, xval + yval);
        vm.pc + 1
    })
}

fn _mod(line: &str) -> Box<Fn(&mut Vm) ->  usize> {
    let parts : Vec<String> = line.split(" ").map(|x| String::from(x)).collect();
    Box::new(move |vm: &mut Vm| {
        let reg = parts[1].as_bytes()[0] as char;
        let xval = *vm.reg.get(&reg).unwrap();
        let yval = vm.deref_string(&parts[2]);
        vm.reg.insert(reg, xval % yval);
        vm.pc + 1
    })
}

fn jgz(line: &str) -> Box<Fn(&mut Vm) ->  usize> {
    let parts : Vec<String> = line.split(" ").map(|x| String::from(x)).collect();
    Box::new(move |vm: &mut Vm| {
        let xval = vm.deref_string(&parts[1]);
        let yval = vm.deref_string(&parts[2]);
        if xval > 0 {
            (vm.pc as i64 + yval) as usize
        }
        else {
            vm.pc + 1
        }
    })
}

fn part1(lines: &Vec<String>) {
    let mut vm : Vm = Vm::new(0 );
    let mut prog: Vec<Box<Fn(&mut Vm) -> usize>> = Vec::new();
    for line in lines {
        match &line[..3] {
             "snd" => prog.push(snd(&line)),
             "rcv" => prog.push(rcv(&line)),
             "set" => prog.push(set(&line)),
             "mul" => prog.push(mul(&line)),
             "add" => prog.push(add(&line)),
             "mod" => prog.push(_mod(&line)),
             "jgz" => prog.push(jgz(&line)),
            _ => panic!(format!("Unknown instruction: {}",  line)),
        };
    }
    vm.run(&prog);
}

fn snd2(line: &str, tx: Sender<i64>) -> Box<Fn(&mut Vm) ->  usize> {
    let parts : Vec<String> = line.split(" ").map(|x| String::from(x)).collect();
    Box::new(move |vm: &mut Vm| {
        tx.send(vm.deref_string(&parts[1])).expect("Failed to send");
        vm.send_count += 1;
        vm.pc + 1
    })
}

fn rcv2(line: &str, rx: Arc<Receiver<i64>>) -> Box<Fn(&mut Vm) ->  usize> {
    let parts : Vec<String> = line.split(" ").map(|x| String::from(x)).collect();
    Box::new(move |vm: &mut Vm| {
        let rcv = rx.recv_timeout(Duration::new(1,0));
        match rcv {
            Ok(i) => {
                vm.reg.insert(parts[1].as_bytes()[0] as char, i);
                vm.pc + 1
            },
            Err(e) => {
                println!("Error: {}", e);
                usize::max_value()
            }
        }
        
    })
}

fn create_prog<'a>(lines: Vec<String>, tx: Sender<i64>, rx: Arc<Receiver<i64>>) 
        -> Vec<Box<Fn(&mut Vm) -> usize>> {
    let mut prog: Vec<Box<Fn(&mut Vm) -> usize>> = Vec::new();
    for line in lines {
        match &line[..3] {
            "snd" => prog.push(snd2(&line, tx.clone())),
            "rcv" => prog.push(rcv2(&line, rx.clone())),
            "set" => prog.push(set(&line)),
            "mul" => prog.push(mul(&line)),
            "add" => prog.push(add(&line)),
            "mod" => prog.push(_mod(&line)),
            "jgz" => prog.push(jgz(&line)),
            _ => panic!(format!("Unknown instruction: {}",  line)),
        };
    }
    prog
}

fn part2(lines: Vec<String>) {
    let (tx1, rx1) : (Sender<i64>, Receiver<i64>) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    let l1 = lines.clone();
    let l2 = lines.clone();

    let h1 = thread::spawn(move ||{
        let mut vm : Vm = Vm::new(0);
        let prog = create_prog(l1, tx2, Arc::new(rx1));
        vm.run(&prog);
        println!("Prog0 sent: {} messages", vm.send_count);
    });

    let h2 = thread::spawn(move ||{
        let mut vm : Vm = Vm::new(1);
        let prog = create_prog(l2, tx1, Arc::new(rx2));
        vm.run(&prog);
        println!("Prog1 sent: {} messages", vm.send_count);
    });

    h1.join().expect("thread 1 failed");
    h2.join().expect("thread 2 failed")
}

pub fn run() {
    let file = "input/input18.txt";
    let file = File::open(file).expect(&format!("File not found {}", file));
    let file = BufReader::new(&file);
    let lines : Vec<String> = file.lines().map(|l| l.unwrap()).collect();
    part1(&lines);
    part2(lines);
}