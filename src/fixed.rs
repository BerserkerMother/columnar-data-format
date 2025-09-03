// a column is a structure that allows all operation on data. read, write, update, and delete .
// lets imagine an isize column
// what should be the internal representation? a naive way would be a vec. it is very good for
// reading and writing however, update and delete would be costly, especially if we are deleting
// multiple records from it.

// I know that I need another bitvec kinda of thing for storing values null or I could just use
// Option<isize>. Option variant takes one addtional byte to store info however a bitvec takes 1/8
// that(one bit).

// bitvec would make things more complicated because the original Vec would not have that value in
// it. This means we could have 100 records but the data vec be of size 90(because 10 are null).
// Nevertheless, it is bad design to complicate things. We must keep it simple if possible. And
// there would be few instances of null which is normal and part of user data!
// Maybe if our data was sparse in nature it would be feasiable, but for now let's keep it simple!

// Null values can of course be presenet in vec and set to default(0 for isize) but that would
// waste 8 bytes! Making them Option<T>. It takes 1 + 8 byte(without considering alignment), So I am going with bitvec
// thingy. For now, I use a byte as a bit(wasting 7bit per record :)) and then improve it.

// Lets to an int and float type def.

pub type IntArray = Fixed<i32>;

pub type FloatArray = Fixed<f32>;

use std::fmt::Debug;

use crate::bitvec;
use crate::bitvec::BitVec;

pub struct Fixed<T> {
    name: String,  // 24 byets
    data: Vec<T>,  // 24 bytes
    nulls: BitVec, // 24 bytes
}

impl<T> Default for Fixed<T> {
    fn default() -> Self {
        Fixed {
            name: String::default(),
            data: vec![],
            nulls: BitVec::default(),
        }
    }
}

impl Fixed<i32> {
    pub fn test_new() -> Fixed<i32> {
        Fixed {
            name: "test col".to_string(),
            data: vec![1, 2, 3, 0, 0, 0, 4, 5, 0, 6],
            nulls: bitvec![
                true, true, true, false, false, false, true, true, true, true
            ],
        }
    }
}

impl Fixed<f32> {
    pub fn test_new() -> Fixed<f32> {
        Fixed {
            name: "test col".to_string(),
            data: vec![
                1.123f32, 2f32, 3f32, 0f32, 0f32, 0f32, 4f32, 5f32, 0f32, 6f32,
            ],
            nulls: bitvec![
                true, true, true, false, false, false, true, true, true, true
            ],
        }
    }
}

impl<T> Fixed<T> {
    pub fn new(name: String) -> Fixed<T> {
        Fixed {
            name,
            ..Default::default()
        }
    }

    // it should have some way of getting metadata like name. lets go for name for now.
    pub fn name(&self) -> &str {
        &self.name
    }

    // we always add record the end of data.
    pub fn add_record(&mut self, record: T) {
        self.data.push(record);
        self.nulls.push(true);
    }

    // somehow I have to get records, I would just allocate new buffer each time for now.

    // there is no need to allocate buffer here. Just use another struct for viewing.
    pub fn get_records(&self) -> FixedViewer<'_, T> {
        FixedViewer {
            inner: self,
            index: 0,
        }
    }

    // updating is a bit tricky because we must calculate wether the element is null for not
    // when updating null, we have to somehow make it appear there and this costly! moving vec
    // elements, so??? maybe a HashMap? nope.
    // Ok. This is solved because because we just allocated data space for null values
    // too.(simplicity)
    pub fn update_record(&mut self, id: usize, record: Option<T>) -> Result<(), &'static str> {
        if id > self.data.len() - 1 {
            return Err("bad id");
        }

        if let Some(record) = record {
            self.data[id] = record;
            self.nulls.set(id, true);
        } else {
            // self.data[id] = 0;
            self.nulls.set(id, false);
        }
        Ok(())
    }

    // now for deleting, we will just swap the value with the last value and delete the it then.
    pub fn delete_record(&mut self, id: usize) -> Result<(), &'static str> {
        if id > self.data.len() - 1 {
            return Err("bad id");
        }
        // ASM check, what happens if I define local parameter?
        let last_index = self.data.len() - 1;
        self.data.swap(id, last_index);
        self.data.pop();
        self.nulls.swap(id, last_index);
        self.nulls.pop();

        Ok(())
    }
}

pub struct FixedViewer<'a, T> {
    inner: &'a Fixed<T>,
    index: usize, // for iterating
}

impl<'a, T: Copy> Iterator for FixedViewer<'a, T> {
    type Item = Option<T>; // returns Options because it can be null
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.inner.data.len() {
            return None;
        }

        let not_null = self.inner.nulls[self.index];
        let value = {
            if not_null {
                Some(self.inner.data[self.index])
            } else {
                None
            }
        };
        self.index += 1;
        Some(value)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.index = n;
        self.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.inner.data.len() - self.index;
        (size, Some(size))
    }
}

impl<'a, T: Copy> ExactSizeIterator for FixedViewer<'a, T> {}

//
// lets have debug way of seeing the column for dev
impl<'a> Debug for FixedViewer<'a, f32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}\n--------", self.inner.name())?;
        for (index, not_null) in self.inner.nulls.iter().enumerate() {
            if not_null {
                writeln!(f, "{:3.3}", &self.inner.data[index])?;
            } else {
                writeln!(f, "<NULL>")?;
            }
        }
        Ok(())
    }
}

// lets have debug way of seeing the column for dev
impl<'a> Debug for FixedViewer<'a, i32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}\n--------", self.inner.name())?;
        for (index, not_null) in self.inner.nulls.iter().enumerate() {
            if not_null {
                writeln!(f, "{}", &self.inner.data[index])?;
            } else {
                writeln!(f, "<NULL>")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{bitvec, bitvec::BitVec};

    use super::Fixed;

    #[test]
    fn add_record() {
        let mut col = Fixed::default();
        col.add_record(5);
        col.add_record(10);
        col.add_record(40);
        col.add_record(80);

        assert_eq!(col.data, vec![5, 10, 40, 80]);
        assert_eq!(col.nulls, bitvec![true, true, true, true]);
    }

    #[test]
    fn delete_record() {
        let mut col = Fixed {
            name: "test col".to_string(),
            data: vec![1, 2, 3, 0, 0, 0, 4, 5, 0, 6],
            nulls: bitvec![
                true, true, true, false, false, false, true, true, false, true
            ],
        };

        col.delete_record(3).unwrap();

        assert_eq!(col.data, vec![1, 2, 3, 6, 0, 0, 4, 5, 0]);
        assert_eq!(
            col.nulls,
            bitvec![true, true, true, true, false, false, true, true, false]
        );
    }

    #[test]
    fn update_record() {
        let mut col = Fixed {
            name: "test col".to_string(),
            data: vec![1, 2, 3, 0, 0, 0, 4, 5, 0, 6],
            nulls: bitvec![
                true, true, true, false, false, false, true, true, false, true
            ],
        };

        col.update_record(3, Some(9)).unwrap();

        assert_eq!(col.data, vec![1, 2, 3, 9, 0, 0, 4, 5, 0, 6]);
        assert_eq!(
            col.nulls,
            bitvec![
                true, true, true, true, false, false, true, true, false, true
            ]
        );
    }

    // #[test]
    // fn get_record() {
    //     let col = Fixed {
    //         name: "test col".to_string(),
    //         data: vec![1, 2, 3, 0, 0, 0, 4, 5, 0, 6],
    //         nulls: bitvec![
    //             true, true, true, false, false, false, true, true, false, true
    //         ],
    //     };
    //
    //     let records = col.get_records();
    //     assert_eq!(
    //         records,
    //         vec![
    //             Some(&1),
    //             Some(&2),
    //             Some(&3),
    //             None,
    //             None,
    //             None,
    //             Some(&4),
    //             Some(&5),
    //             None,
    //             Some(&6)
    //         ]
    //     );
    // }
}
