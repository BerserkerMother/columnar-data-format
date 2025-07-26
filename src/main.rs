use database::Database;
use metadata::Metadata;

mod database;
mod metadata;
mod numeric;
mod storage;
mod text;

fn main() {
    // let actual = Database {
    //     meta: Metadata::new(3, vec!["hello".into(), "there".into(), "you".into()]),
    //     fields: vec![],
    // };
    // actual.to_disk("data").unwrap();
    let db = Database::new("data").unwrap();
    dbg!(&db);
}
