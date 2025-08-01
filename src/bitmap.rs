#[derive(Debug, Default)]
pub struct BitVec {
    inner: Vec<u8>, // 24 byte
    length: usize,  // 8 byte
}

impl BitVec {
    pub fn new() -> BitVec {
        BitVec::default()
    }

    pub fn push(&mut self, value: bool) {
        let bucket = self.length / 8;
        let index = self.length % 8;
        if index == 0 {
            // need to allocate a u8
            self.inner.push(0);
        }

        self.length += 1;
        if value {
            self.inner[bucket] |= 1 << index
        } else {
            self.inner[bucket] &= !(1 << index)
        }
    }

    pub fn pop(&mut self) -> bool {
        let length = self.length - 1;
        let bucket = length / 8;
        let index = length % 8;
        let value = (self.inner[bucket] >> index) & 1;
        if index == 0 {
            self.inner.pop().unwrap();
        }
        self.length -= 1;
        value == 1
    }

    pub fn get(&mut self, index: usize) -> Option<bool> {
        let bucket = index / 8;
        if bucket >= self.inner.len() {
            return None;
        }
        let index = index % 8;

        let value = (self.inner[bucket] >> index) & 1;
        Some(value == 1)
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        let bucket_a = a / 8;
        let index_a = a % 8;
        let bucket_b = b / 8;
        let index_b = b % 8;

        let value_a = (self.inner[bucket_a] >> index_a) & 1;
        let value_b = (self.inner[bucket_b] >> index_b) & 1;

        // if values are not equal, the we can swap them by flipping them
        // this doesn't affect other elements because in xor,
        // when bit a is zero, result is bit b and when bit a is one, result is not b
        // so we only set the place we want to change to one.
        if value_a != value_b {
            self.inner[bucket_a] ^= 1 << index_a;
            self.inner[bucket_b] ^= 1 << index_b;
        }
    }
}

#[cfg(test)]
mod test {
    use super::BitVec;

    #[test]
    fn push() {
        let mut vec = BitVec::new();
        vec.push(true);
        vec.push(false);
        vec.push(false);
        vec.push(true);
        vec.push(true);
        vec.push(false);

        assert_eq!(vec.inner, vec![0b00011001]);

        vec.push(true);
        vec.push(true);
        vec.push(true);
        assert_eq!(vec.inner, vec![0b11011001, 0b00000001]);
    }

    #[test]
    fn pop() {
        let mut vec = BitVec::new();
        vec.inner = vec![0xFF, 0xF5, 0x42]; // 11111111 11110101 01000010
        vec.length = 24;
        dbg!(&vec);
        assert_eq!(vec.pop(), false);
        assert_eq!(vec.pop(), true);
        assert_eq!(vec.pop(), false);
        assert_eq!(vec.pop(), false);
        assert_eq!(vec.pop(), false);
        assert_eq!(vec.pop(), false);
        assert_eq!(vec.pop(), true);
        assert_eq!(vec.pop(), false);
        assert_eq!(vec.pop(), true);
        assert_eq!(vec.pop(), true);
        assert_eq!(vec.pop(), true);
        assert_eq!(vec.pop(), true);
        assert_eq!(vec.pop(), false);
        assert_eq!(vec.pop(), true);
        assert_eq!(vec.pop(), false);
        assert_eq!(vec.pop(), true);
        assert_eq!(vec.pop(), true);
        assert_eq!(vec.pop(), true);
        assert_eq!(vec.pop(), true);
        assert_eq!(vec.pop(), true);
        assert_eq!(vec.pop(), true);
        assert_eq!(vec.pop(), true);
        assert_eq!(vec.pop(), true);
        assert_eq!(vec.pop(), true);
    }

    #[test]
    fn swap() {
        let mut vec = BitVec::new();
        vec.inner = vec![0xFF, 0xF5, 0x42]; // 11111111 11110101 01000010
        vec.length = 24;

        vec.swap(23, 17);
        assert_eq!(vec.inner, vec![0xFF, 0xF5, 0b11000000]);

        vec.swap(4, 9);
        assert_eq!(vec.inner, vec![0b11101111, 0b11110111, 0b11000000]);
    }

    #[test]
    fn get_some() {
        let mut vec = BitVec::new();
        vec.inner = vec![0xFF, 0xF5, 0x42]; // 11111111 11110101 01000010
        vec.length = 24;

        assert_eq!(vec.get(3), Some(true));
        assert_eq!(vec.get(16), Some(false));
    }

    #[test]
    #[should_panic]
    fn get_none() {
        let mut vec = BitVec::new();
        vec.inner = vec![0xFF, 0xF5, 0x42]; // 11111111 11110101 01000010
        vec.length = 24;

        vec.get(24).unwrap();
    }
}
