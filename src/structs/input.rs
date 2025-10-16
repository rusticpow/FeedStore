#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Input {
    pub key_name: String,
    pub cols: Vec<InputCol>,
    pub rows: Vec<InputRow>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputRow {
    pub timestamp: u64,
    pub values: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputCol {
    pub col_name: String,
    pub precision: u8,
}

