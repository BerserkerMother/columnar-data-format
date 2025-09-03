mod bitvec;
mod database;
mod fixed;
mod storage;
mod text;
mod util;

fn main() {
    let col = fixed::FloatArray::test_new();
    println!("{:?}", col.get_records());
}
