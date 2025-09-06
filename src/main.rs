mod bitvec;
mod database;
mod fixed;
mod storage;
mod text;
mod util;
mod variable;

fn main() {
    // let col = fixed::FloatArray::test_new();
    // dbg!(col.get_records().nth(9).unwrap());
    let mut col = variable::Variable::<&str>::new();
    col.add("hahah");
    col.add("wow this world is great");
    dbg!(&col);
    dbg!(col.get(0));
    dbg!(&col);
    col.validity.set(0, false);
    dbg!(col.get(0));
    dbg!(col.get(10));
}
