

const A_START : u64 = 512;
const B_START : u64 = 191;
const A_FAC : u64 = 16807;
const B_FAC : u64 = 48271;
const MOD_VAL : u64 = 2147483647;
const MASK : u64 = (1 << 16) - 1; 

pub fn run() {
    part1();
    part2();
}

fn part1() {
    let mut a = A_START;
    let mut b = B_START;
    let mut match_count = 0;
    for _ in 0..40_000_000 {
        a = next(a, A_FAC);
        b = next(b, B_FAC);
        if matches(a,b) {
            match_count += 1;
        }
    }
    println!("match_count={}", match_count);
}

fn part2() {
    let mut a = A_START;
    let mut b = B_START;
    let mut match_count = 0;
    for _ in 0..5_000_000 {
        a = next(a, A_FAC);
        b = next(b, B_FAC);
        while a % 4 != 0 {
            a = next(a, A_FAC);
        }
        while b % 8 != 0 {
            b = next(b, B_FAC);
        }
        if matches(a,b) {
            match_count += 1;
        }
    }
    println!("match_count={}", match_count);
}

fn next(n: u64, n_fac: u64) -> u64{
    (n * n_fac) % MOD_VAL
}

fn matches(a: u64, b: u64) -> bool {
    (a & MASK) == (b & MASK)
}