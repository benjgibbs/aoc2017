

fn part_one(move_size: usize, num_iterations: usize) -> i32 {
    let mut buffer : Vec<i32>  = Vec::new();
    let mut cur_pos : usize = 0;
    buffer.push(0);
    for i in 0..num_iterations {
        let next_pos = (cur_pos + move_size) % buffer.len();
        buffer.insert(next_pos + 1, (i+1) as i32);
        cur_pos = next_pos + 1;
    }
    buffer[cur_pos+1]
}

fn part_two(move_size: usize, num_iterations: usize) -> u32 {
    let mut buff_size : u32 = 1;
    let mut cur_pos : u32 = 0;
    let mut position_2 : u32  = 0;
    for i in 0..num_iterations {
        let next_pos = (cur_pos + move_size as u32) % buff_size;
        if next_pos == 0 {
            position_2 = (i+1) as u32;
        }
        buff_size += 1;
        cur_pos = next_pos + 1;
    }
    position_2
}

pub fn run() {
    println!("{}", part_one(3, 2017));
    println!("{}", part_one(344, 2017));
    println!("{}", part_two(344, 50000000));
}