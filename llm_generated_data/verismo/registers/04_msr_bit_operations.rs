// Variation: MSR Bit Operations
// From: source/verismo/src/registers/msr_t.rs (BIT64 macro, MSR operations)
// Demonstrates: Bit manipulation for Model-Specific Register values

use vstd::prelude::*;

verus! {

pub open spec fn spec_bit64(x: u64) -> u64 {
    1u64 << x
}

pub fn bit64(x: u64) -> (result: u64)
    requires
        x < 64,
    ensures
        result == spec_bit64(x),
{
    1u64 << x
}

pub fn test_bit(value: u64, bit_pos: u64) -> (result: bool)
    requires
        bit_pos < 64,
    ensures
        result <==> ((value & spec_bit64(bit_pos)) != 0),
{
    (value & bit64(bit_pos)) != 0
}

pub fn set_bit(value: u64, bit_pos: u64) -> (result: u64)
    requires
        bit_pos < 64,
    ensures
        result == value | spec_bit64(bit_pos),
{
    value | bit64(bit_pos)
}

pub fn clear_bit(value: u64, bit_pos: u64) -> (result: u64)
    requires
        bit_pos < 64,
    ensures
        result == value & !spec_bit64(bit_pos),
{
    value & !bit64(bit_pos)
}

pub fn toggle_bit(value: u64, bit_pos: u64) -> (result: u64)
    requires
        bit_pos < 64,
    ensures
        result == value ^ spec_bit64(bit_pos),
{
    value ^ bit64(bit_pos)
}

// Note: Extract and set bits functions with dynamic count are challenging to verify
// due to overflow checking complexities. For production use, consider fixed-width
// field accessors or additional overflow assertions.

pub open spec fn spec_extract_bits(value: u64, start: u64, count: u64) -> u64 {
    let mask = ((1u64 << count) - 1) as u64;
    ((value >> start) & mask) as u64
}

pub open spec fn spec_set_bits(value: u64, start: u64, count: u64, bits: u64) -> u64 {
    let count_mask = ((1u64 << count) - 1) as u64;
    let mask = (count_mask << start) as u64;
    let bits_shifted = (bits << start) as u64;
    ((value & !mask) | (bits_shifted & mask)) as u64
}

pub struct MsrValue {
    pub value: u64,
}

impl MsrValue {
    pub fn new(value: u64) -> (result: Self)
        ensures
            result.value == value,
    {
        MsrValue { value }
    }

    pub fn zero() -> (result: Self)
        ensures
            result.value == 0,
    {
        MsrValue { value: 0 }
    }

    pub fn get(&self) -> (result: u64)
        ensures
            result == self.value,
    {
        self.value
    }

    pub fn set(&mut self, new_value: u64)
        ensures
            self.value == new_value,
    {
        self.value = new_value;
    }

    pub fn test_bit(&self, bit_pos: u64) -> (result: bool)
        requires
            bit_pos < 64,
        ensures
            result <==> ((self.value & spec_bit64(bit_pos)) != 0),
    {
        test_bit(self.value, bit_pos)
    }

    pub fn set_bit(&mut self, bit_pos: u64)
        requires
            bit_pos < 64,
        ensures
            self.value == old(self).value | spec_bit64(bit_pos),
    {
        self.value = set_bit(self.value, bit_pos);
    }

    pub fn clear_bit(&mut self, bit_pos: u64)
        requires
            bit_pos < 64,
        ensures
            self.value == old(self).value & !spec_bit64(bit_pos),
    {
        self.value = clear_bit(self.value, bit_pos);
    }

    pub fn toggle_bit(&mut self, bit_pos: u64)
        requires
            bit_pos < 64,
        ensures
            self.value == old(self).value ^ spec_bit64(bit_pos),
    {
        self.value = toggle_bit(self.value, bit_pos);
    }

    pub open spec fn spec_extract_field(&self, start: u64, count: u64) -> u64 {
        spec_extract_bits(self.value, start, count)
    }

    pub open spec fn spec_set_field(&self, start: u64, count: u64, field_value: u64) -> u64 {
        spec_set_bits(self.value, start, count, field_value)
    }
}

pub fn split_u64_to_u32(value: u64) -> (result: (u32, u32))
{
    #[verifier::truncate]
    let low = value as u32;
    #[verifier::truncate]
    let high = (value >> 32) as u32;
    (low, high)
}

pub fn combine_u32_to_u64(low: u32, high: u32) -> (result: u64)
    ensures
        result == ((high as u64) << 32) | (low as u64),
{
    ((high as u64) << 32) | (low as u64)
}

fn test_msr_bit_operations() {
    let bit_val = bit64(5);

    let value = 0x1234_5678_9ABC_DEF0u64;
    let is_set = test_bit(value, 4);
    let with_bit = set_bit(value, 10);
    let without_bit = clear_bit(value, 4);
    let toggled = toggle_bit(value, 8);

    let mut msr = MsrValue::new(0x1000);
    msr.set_bit(0);
    msr.clear_bit(12);
    msr.toggle_bit(5);
    let test_bit_result = msr.test_bit(0);

    let (low, high) = split_u64_to_u32(value);
    let combined = combine_u32_to_u64(low, high);

    proof {
        let extracted = spec_extract_bits(value, 8, 8);
        let field = msr.spec_extract_field(0, 8);
    }
}

} // verus!

fn main() {
    test_msr_bit_operations();
}
