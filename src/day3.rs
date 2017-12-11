

pub fn run() {
    find(9);
    find(10);
    find(11);
    find(25);
    find(1024);
}

fn found(x: u32, r: u32){
    println!("{}: result={}",x,  r);
}

fn find(find: u32)  {
    let mut n = 3;
    while n*n < find {
        n += 2;
    }
    if n*n == find {
        found(find, n-1);
        return;
    }

    n -= 2;
    //println!("{}, n*n={}", n, n*n);
    let nearest = n*n;
    let distance = find - nearest;
    
    let stride = n + 1;
    let pos = distance % stride;
    if pos - stride/2 == 0 || pos - 3*stride/2 == 0 || pos - 5 * stride / 2|| pos - 5 * stride / 2 == 0 {
        found(find, n-1);
    } else if pos - stride/2  < 0 {
        found(find, n-1 + pos);
    } else if pos - stride < 0 {

    }
    
}