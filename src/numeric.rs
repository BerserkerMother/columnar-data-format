/// numeric column metadata
#[derive(Debug)]
pub struct Numeric {
    data: Vec<isize>,
    meta: NumericMeta,
}

/// numeric column metadata
#[derive(Debug)]
struct NumericMeta {
    path: String,
    length: usize,
}

// byte slice, length
#[derive(Debug)]
struct NumericBuilder<'a> {
    src: &'a [u8],
    length: usize,
}

impl<'a> NumericBuilder<'a> {
    pub fn new(src: &'a [u8]) -> NumericBuilder<'a> {
        // assumes little endian
        let length = usize::from_le_bytes(src[..8].try_into().expect("numeric length"));
        let src = &src[8..]; // skip 8 bytes
        NumericBuilder { src, length }
    }

    pub fn build(&self) -> Vec<isize> {
        let mut vec: Vec<isize> = Vec::with_capacity(self.length);
        for i in 0..self.length {
            let start = i * size_of::<isize>();
            let num = isize::from_le_bytes(
                self.src[start..start + size_of::<isize>()]
                    .try_into()
                    .unwrap(),
            );
            vec.push(num);
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use std::slice;

    use super::*;
    #[test]
    fn numeric_build() {
        let src: [isize; 4] = [3, 5, 10, 32]; // first element is size
        let src_bytes = unsafe {
            slice::from_raw_parts::<'_, u8>(
                src.as_ptr() as *const u8,
                src.len() * size_of::<isize>(),
            )
        };
        let builder = NumericBuilder::new(src_bytes);
        let got = builder.build();

        assert_eq!(vec![5, 10, 32], got);
    }
}
