use std::io::{self, Write};

use crate::util::{self, SliceReader};

// metadata holds additional useful information about data
#[derive(Debug, PartialEq, Eq)]
pub struct Metadata {
    num_fields: usize,
    fields: Vec<FieldMeta>,
}

impl Metadata {
    pub fn new(num_fields: usize, fields: Vec<FieldMeta>) -> Metadata {
        Metadata { num_fields, fields }
    }

    pub fn write_bytes<T: Write>(&self, buffer: &mut T) -> io::Result<()> {
        buffer.write_all(self.num_fields.to_le_bytes().as_slice())?;

        for field in &self.fields {
            field.write_bytes(buffer)?;
        }

        Ok(())
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut reader = SliceReader::new(slice);
        Self::from_reader(&mut reader)
    }

    pub fn from_reader(reader: &mut util::SliceReader) -> Result<Self, Box<dyn std::error::Error>> {
        let num_bytes = reader
            .read_exact(size_of::<usize>())
            .ok_or("buffer too short")?;
        let num_fields = util::read_usize(num_bytes)?;
        let mut fields = vec![];
        for _ in 0..num_fields {
            let field = FieldMeta::from_reader(reader)?;
            fields.push(field);
        }
        Ok(Self { num_fields, fields })
    }

    pub fn num_fields(&self) -> usize {
        self.num_fields
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FieldMeta {
    // 32 bytes because alignment
    name: String,          // 24 bytes
    field_type: FieldType, // 1 byte possibly
}

impl FieldMeta {
    pub fn new(name: String, field_type: FieldType) -> Self {
        FieldMeta { name, field_type }
    }

    fn write_bytes<T: Write>(&self, buf: &mut T) -> io::Result<()> {
        buf.write_all(self.name.len().to_le_bytes().as_ref())?;
        buf.write_all(self.name.as_bytes())?;
        buf.write_all(&[self.field_type as u8])?;
        Ok(())
    }

    fn from_reader(reader: &mut util::SliceReader) -> Result<Self, Box<dyn std::error::Error>> {
        let num_bytes = reader
            .read_exact(size_of::<usize>())
            .ok_or("buffer too short")?;
        let length = util::read_usize(num_bytes)?;

        let field_bytes = reader.read_exact(length + 1).ok_or("buffer too short")?;
        let name = str::from_utf8(&field_bytes[..length])?;
        let type_byte = field_bytes[length]; // type_field
        let field_type = FieldType::try_from(type_byte)?;

        Ok(FieldMeta {
            name: name.to_string(),
            field_type,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FieldType {
    Numeric = 0,
    Text = 1,
}

impl TryFrom<u8> for FieldType {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FieldType::Numeric),
            1 => Ok(FieldType::Text),
            _ => Err("bad field type"),
        }
    }
}
