// Variation 13: Basic Bit Operations
// From: source/verismo/src/tspec_e/math/bits_e.rs
// Demonstrates: Bit manipulation, bitwise operators

use vstd::prelude::*;

verus! {

pub fn bit_check(value: u64, bit: u32) -> (result: bool)
    requires
        bit < 64,
{
    (value & (1u64 << bit)) != 0
}

pub fn bit_set(value: u64, bit: u32) -> (result: u64)
    requires
        bit < 64,
{
    value | (1u64 << bit)
}

pub fn bit_clear(value: u64, bit: u32) -> (result: u64)
    requires
        bit < 64,
{
    value & !(1u64 << bit)
}

pub fn bit_toggle(value: u64, bit: u32) -> (result: u64)
    requires
        bit < 64,
{
    value ^ (1u64 << bit)
}

pub fn count_set_bits_u8(value: u8) -> (result: u8)
    ensures
        result <= 8,
{
    let mut count = 0;
    let mut v = value;
    let mut i = 0;
    while i < 8
        invariant
            0 <= i <= 8,
            count <= i,
        decreases 8 - i
    {
        if v & 1 == 1 {
            count = count + 1;
        }
        v = v >> 1;
        i = i + 1;
    }
    count
}

fn test_bit_ops() {
    let val = 0b1010u64;

    let set_result = bit_set(val, 2);
    let clear_result = bit_clear(val, 1);

    let count = count_set_bits_u8(0b10101010);
    assert(count <= 8);
}

} // verus!

fn main() {
    test_bit_ops();
}
