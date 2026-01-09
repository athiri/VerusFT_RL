// Variation 14: Type Conversions
// From: source/verismo/src/tspec_e/math/bits_e.rs
// Demonstrates: Bit shifting, type conversions

use vstd::prelude::*;

verus! {

pub fn u16_to_u32(low: u16, high: u16) -> (result: u32) {
    (low as u32) | ((high as u32) << 16)
}

pub fn u32_high_u16(value: u32) -> (result: u16) {
    (value >> 16) as u16
}

pub fn u32_low_u16(value: u32) -> (result: u16) {
    value as u16
}

pub fn split_u32(value: u32) -> (result: (u16, u16)) {
    let low = value as u16;
    let high = (value >> 16) as u16;
    (low, high)
}

pub fn u8_to_u16(low: u8, high: u8) -> (result: u16) {
    (low as u16) | ((high as u16) << 8)
}

pub fn u16_to_u8s(value: u16) -> (result: (u8, u8)) {
    let low = value as u8;
    let high = (value >> 8) as u8;
    (low, high)
}

fn test_conversions() {
    let combined = u16_to_u32(0x1234, 0x5678);
    let high = u32_high_u16(combined);
    let low = u32_low_u16(combined);

    let (l, h) = split_u32(combined);

    let val16 = u8_to_u16(0x12, 0x34);
    let (b0, b1) = u16_to_u8s(val16);
}

} // verus!

fn main() {
    test_conversions();
}
