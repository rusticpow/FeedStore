use std::vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PieceColumn {
    pub column_name: String,
    pub timestamps: Vec<u64>,
    pub values: Vec<i64>,
    pub precisions: Vec<u8>,
}

impl PieceColumn {
    pub fn new(column_name: &str) -> PieceColumn {
        PieceColumn {
            column_name: column_name.to_owned(),
            precisions: vec![],
            timestamps: vec![],
            values: vec![],
        }
    }

    pub fn add(&mut self, value: i64, timestamp: u64, precision: u8) {
        let last_ts = self.timestamps.last();
        if let Some(last_ts) = last_ts {
            if *last_ts > timestamp {
                let insert_index = insert_index(&self.timestamps, timestamp);
                self.timestamps.insert(insert_index, timestamp);
                self.values.insert(insert_index, value);
                self.precisions.insert(insert_index, precision);

                return;
            }
        }

        self.timestamps.push(timestamp);
        self.values.push(value);
        self.precisions.push(precision);
    }

    pub fn get_exact(&self, timestamp: u64) -> Option<(i64, u8)> {
        let timestamp_index = self.timestamps.binary_search(&timestamp);
        if let Ok(index) = timestamp_index {
            let value = self.values[index];
            let precision = self.precisions[index];
            return Some((value, precision));
        }

        None
    }
}

fn insert_index(timestamps: &[u64], search_timestamp: u64) -> usize {
    match timestamps.binary_search(&search_timestamp) {
        Ok(index) => index + 1,
        Err(index) => index,
    }
}

#[cfg(test)]
mod tests {
    use super::PieceColumn;

    #[test]
    pub fn test_add_to_end() {
        let mut piece_column = PieceColumn::new("ask");
        piece_column.add(101, 1, 2);

        assert_eq!(piece_column.timestamps, vec![1]);
        assert_eq!(piece_column.values, vec![101]);
        assert_eq!(piece_column.precisions, vec![2]);

        assert_eq!(piece_column.column_name, "ask");

        let (value, precision) = piece_column.get_exact(1).unwrap();

        assert_eq!((101, 2), (value, precision))
    }

    #[test]
    pub fn test_add_to_middle() {
        let mut piece_column = PieceColumn::new("ask");
        piece_column.add(101, 31, 2);
        piece_column.add(99, 33, 2);
        piece_column.add(103, 32, 2);

        assert_eq!(piece_column.timestamps, vec![31, 32, 33]);
        assert_eq!(piece_column.values, vec![101, 103, 99]);
        assert_eq!(piece_column.precisions, vec![2, 2, 2]);

        let (value, precision) = piece_column.get_exact(32).unwrap();
        assert_eq!((103, 2), (value, precision))
    }
}
