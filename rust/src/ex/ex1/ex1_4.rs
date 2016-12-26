/// n < 0のとき, Noneを返す
/// n >= 0のとき, n以下の最大の二乗数の平方根 Some(k)を返す
/// # Example
/// ```
/// assert_eq!(sqrt_int32(-4), None)
/// assert_eq!(sqrt_int32(0), Some(0))
/// assert_eq!(sqrt_int32(4), Some(2))
/// assert_eq!(sqrt_int32(10001), Some(100))
/// ```
fn sqrt_int32(n: i32) -> Option<i32> {
    if n < 0 {
        return None;
    }

    let f = (n as f32).sqrt();
    let k = f as i32;

    if (k + 1) * (k + 1) <= n {
        return Some(k + 1);
    } else {
        return Some(k);
    }
}


/// nが素数ならば true を, そうでないならば falseを返す
/// # Example
/// ```
/// assert_eq!(is_prime(2), true)
/// assert_eq!(is_prime(10), false)
/// assert_eq!(is_prime(1), false)
/// assert_eq!(is_prime(0), false)
/// assert_eq!(is_prime(-1), false)
/// assert_eq!(is_prime(-5), false)
/// ```
fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }

    for j in 2..(sqrt_int32(n).unwrap() + 1) {
        if n % j == 0 {
            return false;
        }
    }

    return true;
}


///
/// 数列 an: i32 -> i32 の start から endまでで素数になっている部分を
/// vec! [(a_{j_1}, j_1), (a_{j_2}, j_2), ...]
/// の形式で返す.
/// # Example
/// ```
/// TODO: Examplesを書く
/// ```
///
fn get_primes_on_series<F>(an: F, start: i32, end: i32) -> Vec<(i32, i32)>
    where F: Fn(i32) -> i32{
    let mut vec: Vec<(i32, i32)> = Vec::new();
    for j in start..end {
        let aj = an(j);
        if is_prime(aj) {
            vec.push((aj, j));
        }
    }
    vec
}


pub fn ex_b() {
    println!("ex_b ===");
    for tj in get_primes_on_series(|n|n*n-2, 2, 100) {
        println!("a_{} = {} is a prime!", tj.1, tj.0);
    }
}

pub fn ex_c() {
    println!("ex_c ===");
    for tj in get_primes_on_series(|n|n*n-3, 2, 100) {
        println!("a_{} = {} is a prime!", tj.1, tj.0);
    }
}

pub fn main() {
    ex_b();
    ex_c();
}
