/// 非負整数nに対して,
/// nが平方数ならば自然数Option(sqrt(n))を出力する.
/// nが平方数でないならばNoneを出力する.
///
/// # Examples
///
/// ```
/// assert_eq!(Some(4), get_square_elemenet(4));
/// assert_eq!(None, get_square_elemenet(5));
/// ```
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


pub fn run() {
    let max_n = 1000;
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
