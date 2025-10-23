use bincode::config::{self, Configuration};

use crate::structs::{
    encodable::piece_header::PieceHeader,
    piece::{self, Piece},
};

pub struct PieceEncoder;

const VERSION_1: u8 = 1;

impl PieceEncoder {
    pub fn encode_v1(piece: &Piece) {
        let version = VERSION_1;

        let config: Configuration = bincode::config::standard().with_variable_int_encoding();
        let header = header(piece);
        let header_bytes: Vec<u8> = bincode::encode_to_vec(header, config).unwrap();
        let body_bytes: Vec<u8> = encode_body(piece);
    }
    pub fn decode() {
        todo!()
    }
}

fn encode_body(piece: &Piece) -> Vec<u8> {
    let mut body_bytes: Vec<u8> = vec![];
    let keys = piece.keys.read().unwrap();
    let key_names = keys.iter().map(|x| x.name.to_owned()).collect::<Vec<String>>();
    let columns = keys.iter().flat_map(|x| {
        let cols = x.columns.lock().unwrap();
        cols.iter().map(|c| c.column_name.to_owned()).collect::<Vec<String>>()
    }).collect::<Vec<String>>();

    body_bytes
}

fn header(piece: &Piece) -> PieceHeader {
    let min_max = piece.min_max_ts.lock().unwrap().clone();
    let key_names = piece
        .keys
        .read()
        .unwrap()
        .iter()
        .map(|x| x.name.to_owned())
        .collect();

    PieceHeader {
        names: key_names,
        min_ts: min_max.min,
        max_ts: min_max.max,
    }
}

