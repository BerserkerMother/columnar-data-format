use std::{collections::HashMap, fmt::Debug};

use crate::bitvec;
use crate::bitvec::BitVec;

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
