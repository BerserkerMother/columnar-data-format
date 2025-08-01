mod database;
mod numeric;
mod bitmap;
mod storage;
mod text;
mod util;

fn main() {
    let col = database::NumberColumn::test_new();
    println!("{:?}", col);
}
