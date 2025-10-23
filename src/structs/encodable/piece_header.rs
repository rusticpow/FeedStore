use bincode::{Decode, Encode};

#[derive(Debug, Encode, Decode)]
pub struct PieceHeader {
    pub names: Vec<String>,
    pub min_ts: u64,
    pub max_ts: u64,


}
