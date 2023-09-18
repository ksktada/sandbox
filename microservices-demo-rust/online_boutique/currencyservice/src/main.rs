use std::{collections::HashMap, fs::File, io::BufReader};

type CurrencyCode = String;
type Rate = String;

fn parse_currency_conversion() -> HashMap<CurrencyCode, Rate> {
    let file = File::open("currency_conversion.json").unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

fn main() {
    let c = parse_currency_conversion();
    println!("{:?}", c);
}