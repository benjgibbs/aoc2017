use std::str::FromStr;

pub fn run() {
    let input_str: &str = "14,58,0,116,179,16,1,104,2,254,167,86,255,55,122,244";
    let input1: Vec<usize> = input_str.split(",")
                                      .map(|s| usize::from_str(s).unwrap())
                                      .collect();

    let buffer1: Vec<usize> = knot_hash(&input1, 1);
    println!("{}", buffer1[0] * buffer1[1]);

    println!("{}", full_knot_hash(input_str));
}

pub fn full_knot_hash(input_str: &str) -> String {
    let mut input: Vec<usize> = input_str.bytes()
                                         .map(|b| b as usize)
                                         .collect();

    input.append(&mut vec![17, 31, 73, 47, 23]);

    let sparse_hash: Vec<usize> = knot_hash(&input, 64);
    let dense_hash = sparse_to_dense_hash(&sparse_hash);
    let mut result: String = String::new();

    for i in dense_hash {
        result += &format!("{:02x}", i);
    }
    result
}

fn sparse_to_dense_hash(input: &Vec<usize>) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    for i in 0..16 {
        let mut v = input[i * 16];
        for j in 1..16 {
            v ^= input[i * 16 + j];
        }
        result.push(v);
    }
    result
}

fn knot_hash(input: &Vec<usize>, rounds: usize) -> Vec<usize> {
    let mut buffer: Vec<usize> = (0..256).collect();
    let mut pos: usize = 0;
    let mut skip_size: usize = 0;

    for _ in 0..rounds {
        for len in input {
            reverse(&mut buffer, pos, *len);
            pos = (pos + skip_size + len) % buffer.len();
            skip_size += 1;
        }
    }
    buffer
}

fn reverse(buffer: &mut Vec<usize>, pos: usize, len: usize) {
    for i in 0..(len / 2) {
        let p1 = (pos + i) % buffer.len();
        let p2 = (pos + len - 1 - i) % buffer.len();
        let t = buffer[p1];
        buffer[p1] = buffer[p2];
        buffer[p2] = t;
    }
}
