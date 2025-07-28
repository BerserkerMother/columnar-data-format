pub fn read_usize(buf: &[u8]) -> Result<usize, Box<dyn std::error::Error>> {
    dbg!(buf.len());
    if buf.len() < size_of::<usize>() {
        return Err("buffer len parse usize".into());
    }
    Ok(usize::from_le_bytes(buf[..size_of::<usize>()].try_into()?))
}

/// reads values from a buffer as slice
pub struct SliceReader<'a> {
    slice: &'a [u8],
}

impl<'a> SliceReader<'a> {
    pub fn new(slice: &'a [u8]) -> SliceReader<'a> {
        SliceReader { slice }
    }

    pub fn read_exact(&mut self, len: usize) -> Option<&[u8]> {
        if self.slice.len() < len {
            return None;
        }

        let slice = &self.slice[..len];
        self.slice = &self.slice[len..];
        Some(slice)
    }
}
