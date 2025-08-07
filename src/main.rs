mod bitvec;
mod database;
mod fixed;
mod storage;
mod text;
mod util;

fn main() {
    let col = fixed::Fixed::test_new();
    println!("{:?}", col);
}
