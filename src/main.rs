use crate::{
    database::{Column, ColumnType, Database, FieldType, Row},
    fixed::Int8Array,
    variable::StringArray,
};

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
    let col1 = StringArray::new();
    let col2 = Int8Array::new();

    let mut db = Database::new(vec![
        Column::new("text array".to_string(), ColumnType::Text(col1)),
        Column::new("int array".to_string(), ColumnType::Int8(col2)),
    ]);

    let random_text = "this life is a joke apache is hard".to_string();
    let row = Row::new(
        vec!["text array".to_string(), "int array".to_string()],
        vec![FieldType::Text(random_text.as_str()), FieldType::Int8(34)],
    );
    db.add_rows(&row);
    db.add_rows(&row);
    db.add_rows(&row);
    db.add_rows(&row);
    db.add_rows(&row);
    db.add_rows(&row);
    db.add_rows(&row);
    db.add_rows(&row);
    db.add_rows(&row);
    db.add_rows(&row);
    db.add_rows(&row);
    db.add_rows(&row);
    db.add_rows(&row);
    // col.add("hahah");
    // col.add("wow this world is great");
    // col.add("wow this world is great");
    // col.add("wow this world is great");
    // col.add("wow this world is great");
    // col.add("wow this world is great");
    // dbg!(col.get(0));
    // col.validity.set(0, false);
    // dbg!(col.get(0));
    // dbg!(col.get(10));
}
