// Variable buffers can store values which doesn't have the same size e.g. strings.
// they consist of 3 buffer, validity bitmap, data, and offset buffer.
// validity is self explainatory. The data buffer holds raw bytes for data and the offset
// indicates where each value start and ends. the first value in offset buffer is 0 and after that
// each offset indicates where the nth value ends. for example, [0, 4, 10] meangs we have two
// values; there first one is 0..4 bytes in the data buffer and the second is 4..10. subsequently
// length of first element is 4-0=4 and the second 10-4=6

pub trait FromByteRef {
    fn from_bytes(bytes: &[u8]) -> &Self;
}

pub trait ToBytesRef {
    fn to_bytes(&self) -> &[u8];
}

impl<T: AsRef<[u8]> + ?Sized> ToBytesRef for T {
    fn to_bytes(&self) -> &[u8] {
        self.as_ref()
    }
}

use std::marker::PhantomData;

use crate::bitvec::BitVec;

#[derive(Debug)]
pub struct Variable<T: ?Sized> {
    data: Vec<u8>,
    offset: Vec<usize>,
    pub validity: BitVec,
    _phantom: PhantomData<T>,
}

// dervie(Default) asks for T: Default in other impls
impl<T: ?Sized> Default for Variable<T> {
    fn default() -> Self {
        Variable {
            data: Default::default(),
            offset: vec![0], // 0 is always there
            validity: Default::default(),
            _phantom: Default::default(),
        }
    }
}

impl<T: ToBytesRef + ?Sized> Variable<T> {
    pub fn new() -> Variable<T> {
        Variable::default()
    }

    pub fn add(&mut self, item: &T) {
        let bytes: &[u8] = item.to_bytes();
        let length = bytes.len() + self.offset.last().unwrap();

        self.data.extend(bytes);
        self.offset.push(length);
        self.validity.push(true);
    }
}

impl<T: FromByteRef + ?Sized> Variable<T> {
    pub fn get(&self, index: usize) -> Option<Option<&T>> {
        if self.offset.len() - 1 < index {
            return None;
        }
        // check validity
        if let Some(not_null) = self.validity.get(index) {
            if !not_null {
                return Some(None);
            }
        } else {
            panic!("validity bitvec and data length missmatch!")
        }

        let (start, end) = (self.offset[index], self.offset[index + 1]);
        let item = T::from_bytes(&self.data[start..end]);
        Some(Some(item))
    }
}

impl FromByteRef for str {
    fn from_bytes(bytes: &[u8]) -> &Self {
        unsafe { str::from_utf8_unchecked(bytes) }
    }
}
