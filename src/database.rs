use std::{collections::HashMap, fmt::Debug};

// columnar database consists of columns and a way to reach them fast.
// so I am going with a HashMap to list name -> column
#[derive(Debug, Default)]
pub struct Database {
    map: HashMap<String, String>,
}

impl Database {
    pub fn new() -> Database {
        Database::default()
    }
}

// a column is a structure that hold allows all operation on data. read, write, update, and delete .
// lets image an isize column
// what should be the internal representation? a naive way would be a vec. it is very good for
// reading and writing however, update and delete would be costly, especially if we are deleting
// multiple records from it.

// I know that I need another bitvec kinda of thing for storing values null or I could just use
// Option<isize>. Option variant takes one addtional byte to store info however a bitvec takes 1/8
// that(one bit).
// bitvec would make things more complicated because the original Vec would not have that value in
// it. This means we could have 100 records but the data vec be of size 90(because 10 are null).
// These values can ofcourse be presenet in vec and set to default(0 for isize) but that would
// waste 8 bytes! Option takes 1 + 8 byte even of the variant in None(without considering alignment), So I am going with bitvec
// thing. For now, I use a byte as a bit(wasting 7bit per record :)) and then improve it.

#[derive(Default)]
pub struct NumberColumn {
    name: String,
    data: Vec<isize>,
    nulls: Vec<u8>,
}

impl NumberColumn {
    pub fn new(name: String) -> NumberColumn {
        NumberColumn {
            name,
            ..Default::default()
        }
    }

    pub fn test_new() -> NumberColumn {
        NumberColumn {
            name: "test col".to_string(),
            data: vec![1, 2, 3, 4, 5, 6],
            nulls: vec![1, 1, 1, 0, 0, 0, 1, 1, 0, 1],
        }
    }

    // it should have some way of getting metadata like name. lets go for name for now.
    pub fn name(&self) -> &str {
        &self.name
    }

    // we always add record the end of data.
    pub fn add_record(&mut self, record: isize) {
        self.data.push(record);
        self.nulls.push(1);
    }

    // somehow I have to get records, I would just allocate new buffer each time for now.
    pub fn get_records(&self) -> Vec<Option<isize>> {
        let mut data = Vec::with_capacity(self.nulls.len());
        let mut index = 0;
        // ASM CHECK: whats the difference between iterating over isize ref and isize
        for null in &self.nulls {
            if *null == 0 {
                data.push(None);
            } else {
                data.push(Some(self.data[index]));
                index += 1;
            }
        }
        data
    }

    // updating is a bit tricky because we must calculate wether the element is null for not
    // when updating null, we have to somehow make it appear there and this costly! moving vec
    // elements, so??? maybe a HashMap? nope. 
}

// lets have debug way of seeing the column for dev
impl Debug for NumberColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}\n----\n", self.name())?;
        let records = self.get_records();
        for rec in records {
            if let Some(rec) = rec {
                writeln!(f, "{}\n", rec)?;
            } else {
                writeln!(f, "null\n")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::NumberColumn;

    #[test]
    fn add_record() {
        let mut col = NumberColumn::default();
        col.add_record(5);
        col.add_record(10);
        col.add_record(40);
        col.add_record(80);

        assert_eq!(col.data, vec![5, 10, 40, 80]);
        assert_eq!(col.nulls, vec![1, 1, 1, 1]);
    }

    #[test]
    fn get_record() {
        let col = NumberColumn {
            name: "".to_string(),
            data: vec![1, 2, 3, 4, 5, 6],
            nulls: vec![1, 1, 1, 0, 0, 0, 1, 1, 0, 1],
        };

        let records = col.get_records();
        assert_eq!(
            records,
            vec![
                Some(1),
                Some(2),
                Some(3),
                None,
                None,
                None,
                Some(4),
                Some(5),
                None,
                Some(6)
            ]
        );
    }
}
