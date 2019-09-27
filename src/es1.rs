extern crate rustc_serialize as serialize;

use es1::serialize::base64::{self, ToBase64};
use es1::serialize::hex::FromHex;
use std::borrow::Cow;

pub fn execute() {
    let input = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    println!("Hex input: {:?}", input);
    let result = "";
    hex_tobase64(&input, &result);
    println!("Base64 output: {:?}", result);
}

pub fn hex_tobase64(input: &str, result: &str) {
    result = &input.from_hex().unwrap().to_base64(base64::STANDARD);
}