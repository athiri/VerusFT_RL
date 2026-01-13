// Variation: ASCII Character Validation
// From: source/verismo/src/debug/ghcb_print.rs
// Demonstrates: ASCII digit/hex validation and conversion

use vstd::prelude::*;

verus! {

pub open spec fn is_ascii_digit(val: u8) -> bool {
    '0' as u8 <= val && val <= '9' as u8
}

pub open spec fn is_ascii_hex_lower(val: u8) -> bool {
    'a' as u8 <= val && val <= 'f' as u8
}

pub open spec fn is_ascii_hex_upper(val: u8) -> bool {
    'A' as u8 <= val && val <= 'F' as u8
}

pub open spec fn is_ascii_hex(val: u8) -> bool {
    is_ascii_digit(val) || is_ascii_hex_lower(val) || is_ascii_hex_upper(val)
}

pub fn check_ascii_digit(val: u8) -> (result: bool)
    ensures
        result <==> is_ascii_digit(val),
{
    '0' as u8 <= val && val <= '9' as u8
}

pub fn check_ascii_hex_lower(val: u8) -> (result: bool)
    ensures
        result <==> is_ascii_hex_lower(val),
{
    'a' as u8 <= val && val <= 'f' as u8
}

pub fn check_ascii_hex_upper(val: u8) -> (result: bool)
    ensures
        result <==> is_ascii_hex_upper(val),
{
    'A' as u8 <= val && val <= 'F' as u8
}

pub fn check_ascii_hex(val: u8) -> (result: bool)
    ensures
        result <==> is_ascii_hex(val),
{
    check_ascii_digit(val) || check_ascii_hex_lower(val) || check_ascii_hex_upper(val)
}

pub fn digit_to_value(c: u8) -> (result: Option<u8>)
    ensures
        match result {
            Some(v) => is_ascii_digit(c) && v < 10,
            None => !is_ascii_digit(c),
        },
{
    if check_ascii_digit(c) {
        Some(c - '0' as u8)
    } else {
        None
    }
}

pub fn hex_to_value(c: u8) -> (result: Option<u8>)
    ensures
        match result {
            Some(v) => is_ascii_hex(c) && v < 16,
            None => !is_ascii_hex(c),
        },
{
    if check_ascii_digit(c) {
        Some(c - '0' as u8)
    } else if check_ascii_hex_lower(c) {
        Some(c - 'a' as u8 + 10)
    } else if check_ascii_hex_upper(c) {
        Some(c - 'A' as u8 + 10)
    } else {
        None
    }
}

fn test_ascii_validation() {
    let is_digit_5 = check_ascii_digit('5' as u8);
    let is_digit_a = check_ascii_digit('a' as u8);

    let is_hex_5 = check_ascii_hex('5' as u8);
    let is_hex_a = check_ascii_hex('a' as u8);
    let is_hex_A = check_ascii_hex('A' as u8);
    let is_hex_g = check_ascii_hex('g' as u8);

    let val1 = digit_to_value('5' as u8);
    let val2 = digit_to_value('a' as u8);

    let hex1 = hex_to_value('5' as u8);
    let hex2 = hex_to_value('a' as u8);
    let hex3 = hex_to_value('F' as u8);
}

} // verus!

fn main() {
    test_ascii_validation();
}
