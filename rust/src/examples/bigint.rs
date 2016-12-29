extern crate num;
use self::num::bigint::ToBigInt;
use self::num::pow;


pub fn main() {
    let x = pow(3.to_bigint().unwrap(), 27);
    println!("{}", x);
}
