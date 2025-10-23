use std::sync::{Arc, Mutex};

use super::{min_max_ts::MinMaxTs, piece_column::PieceColumn};
use super::input::Input;

#[derive(Debug, Clone)]
pub struct PieceKey {
    pub name: String,
    pub columns: Arc<Mutex<Vec<PieceColumn>>>,
}

impl PieceKey {
    pub fn new(name: &str) -> PieceKey {
        PieceKey {
            name: name.to_owned(),
            columns: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn add_records(&self, input: &Input) -> MinMaxTs {
        let mut min_max = MinMaxTs::new();
        let mut columns = self.columns.lock().unwrap();
        for input_row in &input.rows {
            for (i, input_col) in input.cols.iter().enumerate() {
                let piece_col = match 
                    columns
                    .iter_mut()
                    .find(|x| x.column_name.eq(&input_col.col_name))
                {
                    Some(piece_col) => piece_col,
                    None => {
                        let col = PieceColumn::new(&input_col.col_name);
                        columns.push(col);
                        columns.last_mut().unwrap()
                    }
                };

                piece_col.add(
                    input_row.col_values[i],
                    input_row.timestamp,
                    input_col.precision,
                );
                min_max.set(input_row.timestamp);
            }
        }

        min_max
    }

    pub fn get_exact(&self, col: &str, ts: u64) -> Option<(i64, u8)> {
        let columns = self.columns.lock().unwrap();
        if columns.is_empty() {
            return None;
        }

        let column = columns.iter().find(|x| x.column_name.eq(col))?;
        if let Some((val, pre)) = column.get_exact(ts) {
            Some((val, pre))
        } else {
            None
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
                col_values: vec![101],
            }],
        };

        key.add_records(&input);

        let columns = key.columns.lock().unwrap();
        let piece_col = &columns[0];
        let (val, pre) = piece_col.get_exact(33).unwrap();

        assert_eq!((val, pre), (101, 2));
    }
}
