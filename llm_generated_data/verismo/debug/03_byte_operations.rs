// Variation: Byte Array Operations
// From: source/verismo/src/debug/ghcb_print.rs (bytes2u64, str2u64)
// Demonstrates: Converting byte sequences to integers

use vstd::prelude::*;

verus! {

pub fn byte_to_u64(b: u8) -> (result: u64)
    ensures
        result == b as u64,
        result <= 255,
{
    b as u64
}

pub fn combine_two_bytes(b0: u8, b1: u8) -> (result: u64)
    ensures
        result == (b0 as u64) | ((b1 as u64) << 8),
{
    (b0 as u64) | ((b1 as u64) << 8)
}

pub fn extract_byte(val: u64, offset: u8) -> (result: u8)
    requires
        offset < 8,
{
    ((val >> (offset as u64 * 8)) & 0xFF) as u8
}

pub fn pack_bytes_2(bytes: &[u8]) -> (result: u64)
    requires
        bytes@.len() >= 2,
    ensures
        result == (bytes@[0] as u64) | ((bytes@[1] as u64) << 8),
{
    let b0 = bytes[0] as u64;
    let b1 = bytes[1] as u64;
    b0 | (b1 << 8)
}

pub fn pack_bytes_4(bytes: &[u8]) -> (result: u64)
    requires
        bytes@.len() >= 4,
{
    let b0 = bytes[0] as u64;
    let b1 = bytes[1] as u64;
    let b2 = bytes[2] as u64;
    let b3 = bytes[3] as u64;
    b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)
}

pub fn shift_amount(offset: usize) -> (result: u64)
    requires
        offset < 8,
    ensures
        result == offset as u64 * 8,
{
    offset as u64 * 8
}

fn test_byte_operations() {
    let val1 = byte_to_u64(0x42);

    let combined = combine_two_bytes(0x12, 0x34);

    let extracted0 = extract_byte(0x12345678, 0);
    let extracted1 = extract_byte(0x12345678, 1);

    let bytes: [u8; 4] = [0x12, 0x34, 0x56, 0x78];
    let packed2 = pack_bytes_2(&bytes);
    let packed4 = pack_bytes_4(&bytes);

    let shift0 = shift_amount(0);
    let shift2 = shift_amount(2);
}

} // verus!

fn main() {
    test_byte_operations();
}
