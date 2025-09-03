mod bitvec;
mod database;
mod fixed;
mod storage;
mod text;
mod util;

fn main() {
    let col = fixed::FloatArray::test_new();
    dbg!(col.get_records().nth(9).unwrap());
}
