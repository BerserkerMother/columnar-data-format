use std::{io::Write, path::PathBuf};

// metadata holds additional useful information about data
#[derive(Debug, PartialEq, Eq)]
pub struct Metadata {
    num_fields: usize,
    paths: Vec<PathBuf>,
}

impl Metadata {
    pub fn new(num_fields: usize, paths: Vec<PathBuf>) -> Metadata {
        Metadata { num_fields, paths }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![];
        buffer
            .write_all(self.num_fields.to_le_bytes().as_slice())
            .unwrap();

        for path in &self.paths {
            let path = path.to_str().unwrap();
            buffer
                .write_all(path.len().to_le_bytes().as_slice())
                .unwrap();
            buffer.write_all(path.as_bytes()).unwrap();
        }

        buffer
    }
}

#[derive(Debug)]
pub struct FieldMetadata {
    name: String,
    path: PathBuf,
}
