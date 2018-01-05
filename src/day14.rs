
use day10;

pub fn run() {
    let input_string = "amgozmfv";
    let mut used_squares = 0;
    let sz = 128;
    let mut grid : Vec<Vec<char>> = Vec::new();
    let mut regions : Vec<Vec<usize>> = Vec::new();
    for _ in 0..sz {
        let mut region : Vec<usize>  = Vec::new();
        for _ in 0..sz {
            region.push(0);
        }
        regions.push(region);
    }

    for i in 0..sz { 
        let input = String::from(input_string) + "-" +  &i.to_string();
        let knot_hash = day10::full_knot_hash(&input);
        let mut buffer : String = String::new();

        for c in knot_hash.chars() {
            let num = usize::from_str_radix(&c.to_string(), 16).unwrap();
            used_squares += num.count_ones();
            let bits = format!("{:04b}", num);
            buffer.push_str(&bits);
        }
        grid.push(buffer.chars().collect());
    }
    println!("used_squares: {}", used_squares);


    let mut region_count = 0;
    for y in 0..sz {
        for x in 0..sz {            
            if grid[y][x] == '1' && regions[y][x] == 0 {
                region_count += 1;
                mark_neighbours(&grid, &mut regions, x, y, sz, region_count);
            }
        }
    }

    println!("region_count: {}", region_count);
}

fn mark_neighbours(grid: &Vec<Vec<char>>, regions: &mut Vec<Vec<usize>>, 
    x: usize, y: usize, sz: usize, region_count: usize ) {
    for (x2,y2) in adj(x,y,sz) {
        if grid[y2][x2] == '1' && regions[y2][x2] == 0 {
            regions[y2][x2] = region_count;
            mark_neighbours(grid, regions, x2, y2, sz, region_count);
        }
    }
}

fn adj(x: usize, y: usize, sz: usize) -> Vec<(usize,usize)> {
    let mut result = Vec::new();
    if x > 0 {
        result.push((x-1,y));
    }
    if x < (sz-1) {
        result.push((x+1, y));
    }
    if y > 0 {
        result.push((x,y-1));
    }
    if y < (sz-1) {
        result.push((x,y+1));
    }
    result
}