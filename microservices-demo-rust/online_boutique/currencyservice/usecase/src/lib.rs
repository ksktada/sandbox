pub mod dto;

use std::{collections::HashMap, fs::File, io::BufReader};
use domain::r#type::{CurrencyCode, Rate};
use dto::MoneyDto;

fn parse_currency_conversion() -> HashMap<CurrencyCode, Rate> {
    let file = File::open("currency_conversion.json").unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

fn carry(unit: u32, nanos: u32) -> (u32, u32) {
    todo!()
}

pub async fn get_supported_currencies() -> Vec<CurrencyCode> {
    parse_currency_conversion().into_keys().collect::<Vec<CurrencyCode>>()
}

pub async fn convert(from: &MoneyDto, to_code: &CurrencyCode) -> MoneyDto {
    let conversion_map = parse_currency_conversion();
    let euros = conversion_map.get(&from.currency_code);
    MoneyDto {
        currency_code: to_code.clone(),
        units: 1,
        nanos: 1
    }
}