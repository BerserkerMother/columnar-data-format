#[derive(Debug, Default)]
pub struct Text {
    data: Vec<isize>,
    meta: TextMeta,
}

#[derive(Debug, Default)]
struct TextMeta {}
