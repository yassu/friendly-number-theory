fn get_square_elemenet(n: i64) -> Option<i64> {
    if n == 0 || n == 1 {
        return Some(n);
    }

    for j in 1..((n-1)/2 + 1) {
        if j*j == n {
            return Some(j);
        }
    }
    None
}

fn main() {
    let max_n = 10000;
    let mut tri = 0;
    for j in 1..max_n {
        tri += j;
        if let Some(sq) = get_square_elemenet(tri) {
            println!("j = {} ======================================", j);
            println!("tri: {:?}", tri);
            println!("sq:  {:?}", sq);
        }
    }
}
