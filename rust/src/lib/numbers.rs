pub fn get_square_elemenet(n: i64) -> Option<i64> {
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

