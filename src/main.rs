mod database;
mod numeric;
mod storage;
mod text;
mod util;

fn main() {
    let col = database::NumberColumn::test_new();
    println!("{:?}", col);
    dbg!(size_of::<Option<u64>>());
    dbg!(align_of::<usize>());
}
