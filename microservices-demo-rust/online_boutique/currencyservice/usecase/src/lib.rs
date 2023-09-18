pub mod dto;

use std::{collections::HashMap, fs::File, io::BufReader};
use domain::r#type::{CurrencyCode, Rate};
use dto::MoneyDto;

fn parse_currency_conversion() -> HashMap<CurrencyCode, Rate> {
    let file = File::open("currency_conversion.json").unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

pub async fn get_supported_currencies() -> Vec<CurrencyCode> {
    parse_currency_conversion().into_keys().collect::<Vec<CurrencyCode>>()
}

pub async fn convert(from: &MoneyDto, to_code: &CurrencyCode) -> MoneyDto {
    todo!()
}