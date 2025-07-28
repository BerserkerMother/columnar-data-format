use std::io::{Read, Write};
use std::path::Path;
use std::{fs, io};

use log::debug;

use crate::metadata::{FieldMeta, Metadata};
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
        let buffer = bytes.as_slice();
        let meta = Metadata::from_slice(buffer)?;

        let fields = Vec::with_capacity(meta.num_fields());

        Ok(Database { meta, fields })
    }

    pub fn to_disk(&self, path: impl AsRef<Path>) -> io::Result<()> {
        if fs::exists(path.as_ref())? {
            fs::remove_dir_all(path.as_ref())?;
        }
        fs::create_dir(path.as_ref())?;
        let meta_path = path.as_ref().join("meta");
        let mut meta_file = fs::File::create(meta_path)?;
        let mut buffer = vec![];
        self.meta.write_bytes(&mut buffer)?;
        meta_file.write_all(&buffer)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Field {
    meta: FieldMeta,
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

    use crate::metadata::FieldType;

    use super::*;
    #[test]
    fn database_from_bytes_meta() {
        // make actual
        let actual = Database {
            meta: Metadata::new(
                2,
                vec![
                    FieldMeta::new("hello".to_string(), FieldType::Text),
                    FieldMeta::new("there".to_string(), FieldType::Numeric),
                ],
            ),
            fields: vec![],
        };

        let mut buffer: Vec<u8> = vec![];
        buffer.write_all(2usize.to_le_bytes().as_slice()).unwrap();

        buffer.write_all(5usize.to_le_bytes().as_slice()).unwrap();
        buffer.write_all("hello".as_bytes()).unwrap();
        buffer.write_all(&[1]).unwrap();

        buffer.write_all(5usize.to_le_bytes().as_slice()).unwrap();
        buffer.write_all("there".as_bytes()).unwrap();
        buffer.write_all(&[0]).unwrap();

        let got = Database::from_bytes(buffer).unwrap();
        println!("{:?}", &got);
        assert_eq!(actual.meta, got.meta);
    }
}
