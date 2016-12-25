// mod lib;
extern crate lib;
pub use lib::get_square_elemenet;

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
