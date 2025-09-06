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
    let mut col = variable::Variable::<str>::new();
    col.add("hahah");
    col.add("wow this world is great");
    col.add("wow this world is great");
    col.add("wow this world is great");
    col.add("wow this world is great");
    col.add("wow this world is great");
    dbg!(col.get(0));
    col.validity.set(0, false);
    dbg!(col.get(0));
    dbg!(col.get(10));

    let mut col = variable::Variable::<[u8]>::new();
    col.add(b"hahah");
    col.add(b"wow this world is great");
    col.add(b"wow this world is great");
    col.add(b"wow this world is great");
    col.add(b"wow this world is great");
    col.add(b"wow this world is great");
    dbg!(col.get(0));
    col.validity.set(0, false);
    dbg!(col.get(0));
    dbg!(col.get(10));
}
