// Variation: Number to Character Conversion
// From: source/verismo/src/debug/ghcb_print.rs (num_to_char)
// Demonstrates: Converting numeric values to ASCII characters

use vstd::prelude::*;

verus! {

pub open spec fn is_hex_char(val: u8) -> bool {
    ||| '0' as u8 <= val && val <= '9' as u8
    ||| 'a' as u8 <= val && val <= 'f' as u8
}

pub fn num_to_hex_char(n: u8) -> (result: u8)
    requires
        n < 16,
    ensures
        is_hex_char(result),
{
    if n < 10 {
        n + 48  // '0' is ASCII 48
    } else {
        97 + n - 10  // 'a' is ASCII 97
    }
}

pub fn num_to_dec_char(n: u8) -> (result: u8)
    requires
        n < 10,
    ensures
        '0' as u8 <= result && result <= '9' as u8,
{
    n + '0' as u8
}

pub fn hex_char_to_num(c: u8) -> (result: Option<u8>)
    ensures
        match result {
            Some(v) => v < 16 && is_hex_char(c),
            None => !is_hex_char(c),
        },
{
    if '0' as u8 <= c && c <= '9' as u8 {
        Some(c - '0' as u8)
    } else if 'a' as u8 <= c && c <= 'f' as u8 {
        Some(c - 'a' as u8 + 10)
    } else {
        None
    }
}

pub fn get_hex_prefix() -> (result: (u8, u8))
    ensures
        result.0 == '0' as u8,
        result.1 == 'x' as u8,
{
    ('0' as u8, 'x' as u8)
}

pub fn is_valid_hex_digit(n: u8) -> (result: bool)
    ensures
        result <==> n < 16,
{
    n < 16
}

fn test_number_to_char() {
    let c0 = num_to_hex_char(0);
    let c5 = num_to_hex_char(5);
    let c10 = num_to_hex_char(10);
    let c15 = num_to_hex_char(15);

    let d0 = num_to_dec_char(0);
    let d5 = num_to_dec_char(5);
    let d9 = num_to_dec_char(9);

    let n1 = hex_char_to_num('5' as u8);
    let n2 = hex_char_to_num('a' as u8);
    let n3 = hex_char_to_num('g' as u8);

    let prefix = get_hex_prefix();

    let valid1 = is_valid_hex_digit(10);
    let valid2 = is_valid_hex_digit(20);
}

} // verus!

fn main() {
    test_number_to_char();
}
