#[derive(Clone, Debug)]
pub enum Storage {
    Numeric(Vec<isize>),
    Text(Vec<String>),
}
