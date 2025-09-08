// database
// but what is a database. Maybe it can be thought if collection of data with certain schema.
// how to represent a schema? we could have a separate struct to hold schema info, but I don't
// see the value in doing so.
// A database is collection of columns and each column can represent it self. So I am going to make
// a trait that column should implement.

use std::time::Instant;

use crate::{fixed::Fixed, variable::Variable};

// for now, it just hold name and the datatype.
// pub trait Column {
//     fn name(&self) -> &str;
//
//     fn data_type(&self) -> Type;
//
//     fn describe(&self) -> String {
//         format!("{}:{:?}", self.name(), self.data_type())
//     }
//
//     fn get(&self, index: usize) -> Type;
//
//     fn add(&self, value: Type);
// }
//
// #[derive(Debug)]
// pub enum Type<'a> {
//     Int8(i8),
//     Int16(i16),
//     Int32(i32),
//     Int64(i64),
//     UInt8(u8),
//     UInt16(u16),
//     UInt32(u32),
//     UInt64(u64),
//     Float32(f32),
//     Float64(f64),
//     Text(&'a str),
//     ByteArray(&'a [u8]),
// }
//
// a database is just some columns
#[derive(Debug)]
pub struct Database {
    fields: Vec<Column>,
}

impl Database {
    pub fn new(fields: Vec<Column>) -> Database {
        Database { fields }
    }
    pub fn add_rows(&mut self, row: &Row) {
        for (name, f) in row.header.iter().zip(row.fields.iter()) {
            for field in &mut self.fields {
                if &field.name == name {
                    match (&mut field.inner, f) {
                        (ColumnType::Int8(c), &FieldType::Int8(v)) => {
                            c.add_record(v);
                        }
                        (ColumnType::Text(c), &FieldType::Text(v)) => {
                            c.add(v);
                        }
                        _ => panic!("schema miss match"),
                    }
                }
            }
        }
    }
}

pub struct Row<'a> {
    header: Vec<String>,
    fields: Vec<FieldType<'a>>,
}

impl<'a> Row<'a> {
    pub fn new(header: Vec<String>, fields: Vec<FieldType<'a>>) -> Row<'a> {
        Row { header, fields }
    }
}

pub enum FieldType<'a> {
    Int8(i8),
    Text(&'a str),
}

#[derive(Debug)]
pub struct Column {
    name: String,
    inner: ColumnType,
}

impl Column {
    pub fn new(name: String, inner: ColumnType) -> Column {
        Column { name, inner }
    }
    pub fn describe(&self) -> String {
        format!("{}:{}", self.name, self.inner.type_str())
    }
}

#[derive(Debug)]
pub enum ColumnType {
    Int8(Fixed<i8>),
    Text(Variable<str>),
}

impl ColumnType {
    pub fn type_str(&self) -> &'static str {
        match self {
            ColumnType::Int8(_) => "int8",
            ColumnType::Text(_) => "text",
        }
    }
}
