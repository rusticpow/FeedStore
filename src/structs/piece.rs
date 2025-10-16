use std::{
    cmp::{max, min},
    u64,
};

use super::{
    input::{self, Input, InputCol, InputRow},
    piece_column::PieceColumn, piece_key::PieceKey,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PieceHeader {
    pub ts_from: u64,
    pub ts_to: u64,
    pub key_names: Vec<String>,
    pub col_names: Vec<String>,
    pub compressed_size: u32,
    pub uncompressed_size: u32,
    pub compression: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PieceBody {
    pub keys: Vec<PieceKey>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValueColumn {
    start_index: u32,
    records: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piece {
    pub header: PieceHeader,
    pub body: PieceBody,
}

impl Piece {
    pub fn new() -> Piece {
        Piece {
            header: PieceHeader {
                ts_from: u64::MAX,
                ts_to: u64::MIN,
                key_names: vec![],
                col_names: vec![],
                compressed_size: 0,
                uncompressed_size: 0,
                compression: 0,
            },
            body: PieceBody { keys: vec![] },
        }
    }

    pub fn append(&mut self, input: Input) {

        //        self.populate(key_index, input);
    }

    fn populate(&mut self, key_index: usize, mut input: Input) {
        let mut min_ts = u64::MAX;
        let mut max_ts = u64::MIN;

        // add columns if not exist
        for input_col in input.cols.into_iter() {}

        for column in &self.header.col_names {}

        self.header.ts_from = min(self.header.ts_from, min_ts);
        self.header.ts_to = max(self.header.ts_to, max_ts);
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::input::{Input, InputCol, InputRow};

    use super::Piece;

    #[test]
    fn append_input_to_piece() {
        // let input = Input {
        //     key_name: "XAUUSD".to_string(),
        //     cols: vec![
        //         InputCol {
        //             col_name: "ask".to_owned(),
        //             precision: 2,
        //         },
        //         InputCol {
        //             col_name: "bid".to_owned(),
        //             precision: 2,
        //         },
        //     ],
        //     rows: vec![InputRow {
        //         ts: 101,
        //         values: vec![990, 110],
        //     }],
        // };
        //
        // let mut piece = Piece::new();
        // piece.append(input.clone());
        //
        // assert_eq!(piece.header.ts_from, 101);
        // assert_eq!(piece.header.ts_to, 101);
        // assert_eq!(piece.body.name_ts[0], vec![101]);
        // assert_eq!(piece.body.name_values[0]);
        //
        // let piece_from_input = piece.get_by_name("XAUUSD").unwrap();
        //     //
        //     assert_eq!(input, piece_from_input);
    }
}
