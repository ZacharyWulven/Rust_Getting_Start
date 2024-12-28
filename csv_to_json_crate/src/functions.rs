use crate::models::structs::HousePrice;
use csv::{Writer, ReaderBuilder, Reader};


pub fn read_csv(path: String) -> Vec<HousePrice> {
    let mut rdr = Reader::from_path(path).unwrap();
    vec![]
}