use std::sync::{Arc, Mutex, RwLock};

use super::{
    input::Input,
    min_max_ts::MinMaxTs,
    piece_key::{self, PieceKey},
};

#[derive(Debug)]
pub struct Piece {
    pub min_max_ts: Mutex<MinMaxTs>,
    pub compressed_size: u32,
    pub uncompressed_size: u32,
    pub compression: u8,
    pub keys: Arc<RwLock<Vec<PieceKey>>>,
}

impl Piece {
    pub fn new() -> Piece {
        Piece {
            min_max_ts: Mutex::new(MinMaxTs::default()),
            compressed_size: 0,
            uncompressed_size: 0,
            compression: 0,
            keys: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn append(&self, input: &Input) {
        let read = self.keys.read().unwrap();
        let piece_key = read.iter().find(|x| x.name.eq(&input.key_name));

        let piece_key = match piece_key {
            Some(piece_key) => {
                let piece_key = piece_key.clone();
                drop(read);

                piece_key
            }
            None => {
                drop(read);

                let piece_key = PieceKey::new(input.key_name.as_ref());
                let clone = piece_key.clone();

                let mut write = self.keys.write().unwrap();
                write.push(piece_key);

                clone
            }
        };

        let min_max_ts = piece_key.add_records(input);
        let mut min_max_ts_lock = self.min_max_ts.lock().unwrap();
        min_max_ts_lock.merge(&min_max_ts);
    }

    pub fn get_exact(&self, key_name: &str, col_name: &str, ts: u64) -> Option<(i64, u8)> {
        let read = self.keys.read().unwrap();
        let piece_key = read.iter().find(|x| x.name.eq(&key_name));
        match piece_key {
            Some(piece_key) => {
                let piece_key = piece_key.clone();
                drop(read);

                piece_key.get_exact(col_name, ts)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::structs::input::{Input, InputCol, InputRow};

    use super::Piece;

    #[test]
    fn append_input_to_piece() {
        let mut piece = Piece::new();
        let input = Input {
            key_name: "XAUUSD".to_string(),
            cols: vec![InputCol {
                col_name: "ask".to_string(),
                precision: 100,
            }],
            rows: vec![InputRow {
                timestamp: 101,
                col_values: vec![2],
            }],
        };

        piece.append(&input);

        let piece_1 = piece.get_exact("XAUUSD", "ask",  101).unwrap();
        assert_eq!(piece_1, (2, 100));
    }
}
