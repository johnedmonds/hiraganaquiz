pub mod data;
pub mod picker;

#[derive(Debug, PartialEq, Eq)]
pub struct Hiragana<'a> {
    pub jpn: &'a str,
    pub eng: &'a str,
}
