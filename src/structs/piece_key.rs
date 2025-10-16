use std::vec;

use super::{input::Input, piece_column::PieceColumn};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PieceKey {
    pub name: String,
    pub columns: Vec<PieceColumn>,
}

impl PieceKey {
    pub fn new(name: &str) -> PieceKey {
        PieceKey {
            name: name.to_owned(),
            columns: vec![],
        }
    }

    pub fn add_records(&mut self, input: &Input) {
        for input_row in &input.rows {
            for (i, input_col) in input.cols.iter().enumerate() {
                let piece_col = match self
                    .columns
                    .iter_mut()
                    .find(|x| x.column_name.eq(&input_col.col_name))
                {
                    Some(piece_col) => piece_col,
                    None => {
                        let col = PieceColumn::new(&input_col.col_name);
                        self.columns.push(col);
                        self.columns.last_mut().unwrap()
                    }
                };

                piece_col.add(
                    input_row.values[i],
                    input_row.timestamp,
                    input_col.precision,
                );
            }
        }
    }
}

mod tests {
    use crate::structs::{
        input::{Input, InputCol, InputRow},
        piece_key::PieceKey,
    };

    #[test]
    fn test_insert_one_col_one_row() {
        let mut key = PieceKey::new("XAUUSD");

        let input = Input {
            key_name: "XAUUSD".to_owned(),
            cols: vec![InputCol {
                col_name: "ask".to_owned(),
                precision: 2,
            }],
            rows: vec![InputRow {
                timestamp: 33,
                values: vec![101],
            }],
        };

        key.add_records(&input);

        let piece_col = &key.columns[0];
        let (val, pre) = piece_col.get_exact(33).unwrap();

        assert_eq!((val, pre), (101, 2));
    }
}
