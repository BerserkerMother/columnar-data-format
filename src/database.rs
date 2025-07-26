use std::io::{Read, Write};
use std::path::Path;
use std::{fs, io};

use log::debug;

use crate::metadata::{FieldMetadata, Metadata};
use crate::numeric::Numeric;
use crate::text::Text;

#[derive(Debug)]
pub struct Database {
    pub meta: Metadata,
    pub fields: Vec<Field>,
}

impl Database {
    pub fn new(root: impl AsRef<Path>) -> Result<Database, Box<dyn std::error::Error>> {
        // file contains number of fields and their path
        debug!("metadata");
        let mut file = fs::File::open(root.as_ref().join("meta"))?;
        let mut buf = vec![];
        let _ = file.read_to_end(&mut buf)?;

        Self::from_bytes(buf)
    }

    fn from_bytes(bytes: Vec<u8>) -> Result<Database, Box<dyn std::error::Error>> {
        let mut buffer = bytes.as_slice();
        let length = read_usize(buffer)?;
        debug!("length {length}");
        buffer = &buffer[8..];

        let mut paths = Vec::with_capacity(length);

        for _ in 0..length {
            let path_length = read_usize(buffer)?;
            buffer = &buffer[8..];
            let path = str::from_utf8(&buffer[..path_length])?;
            buffer = &buffer[path_length..];
            paths.push(path.into());
        }

        Ok(Database {
            meta: Metadata::new(length, paths),
            fields: vec![],
        })
    }

    pub fn to_disk(&self, path: impl AsRef<Path>) -> io::Result<()> {
        if fs::exists(path.as_ref())? {
            fs::remove_dir_all(path.as_ref())?;
        }
        fs::create_dir(path.as_ref())?;
        let meta_path = path.as_ref().join("meta");
        let mut meta_file = fs::File::create(meta_path)?;
        meta_file.write_all(&self.meta.to_bytes())?;
        Ok(())
    }
}

fn read_usize(buf: &[u8]) -> Result<usize, Box<dyn std::error::Error>> {
    Ok(usize::from_le_bytes(buf[0..8].try_into()?))
}

#[derive(Debug)]
pub struct Field {
    meta: FieldMetadata,
    inner: FieldType,
}

#[derive(Debug)]
pub enum FieldType {
    Numeric(Numeric),
    Text(Text),
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;
    #[test]
    fn database_from_bytes_meta() {
        // make actual
        let actual = Database {
            meta: Metadata::new(3, vec!["hello".into(), "there".into(), "you".into()]),
            fields: vec![],
        };

        let mut buffer: Vec<u8> = vec![];
        buffer.write_all(3usize.to_le_bytes().as_slice()).unwrap();

        buffer.write_all(5usize.to_le_bytes().as_slice()).unwrap();
        buffer.write_all("hello".as_bytes()).unwrap();

        buffer.write_all(5usize.to_le_bytes().as_slice()).unwrap();
        buffer.write_all("there".as_bytes()).unwrap();

        buffer.write_all(3usize.to_le_bytes().as_slice()).unwrap();
        buffer.write_all("you".as_bytes()).unwrap();

        let got = Database::from_bytes(buffer).unwrap();
        println!("{:?}", &got);
        assert_eq!(actual.meta, got.meta);
    }
}
