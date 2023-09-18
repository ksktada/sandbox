pub mod dto;

use std::{collections::HashMap, fs::File, io::BufReader};
use domain::r#type::{CurrencyCode, Rate};

fn parse_currency_conversion() -> HashMap<CurrencyCode, Rate> {
    let file = File::open("currency_conversion.json").unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}