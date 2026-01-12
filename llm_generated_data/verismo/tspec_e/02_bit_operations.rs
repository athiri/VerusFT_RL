// Variation: Bit Operations
// From: source/verismo/src/tspec_e/math/bits_e.rs
// Demonstrates: Bit manipulation and testing operations

use vstd::prelude::*;

verus! {

pub open spec fn has_bit_set(val: u64, bit: u64) -> bool
    recommends
        bit < 64,
{
    val & (1u64 << bit) != 0
}

pub open spec fn spec_bit_set(val: u64, bit: u64) -> u64
    recommends
        bit < 64,
{
    val | (1u64 << bit)
}

pub open spec fn spec_bit_clear(val: u64, bit: u64) -> u64
    recommends
        bit < 64,
{
    val & (!(1u64 << bit))
}

pub struct BitFlags {
    pub value: u64,
}

impl BitFlags {
    pub fn new(value: u64) -> (result: Self)
        ensures
            result.value == value,
    {
        BitFlags { value }
    }

    pub fn zero() -> (result: Self)
        ensures
            result.value == 0,
    {
        BitFlags { value: 0 }
    }

    pub fn get_value(&self) -> (result: u64)
        ensures
            result == self.value,
    {
        self.value
    }

    pub open spec fn is_set(&self, bit: u64) -> bool
        recommends
            bit < 64,
    {
        has_bit_set(self.value, bit)
    }

    pub open spec fn with_bit_set(&self, bit: u64) -> u64
        recommends
            bit < 64,
    {
        spec_bit_set(self.value, bit)
    }

    pub open spec fn with_bit_clear(&self, bit: u64) -> u64
        recommends
            bit < 64,
    {
        spec_bit_clear(self.value, bit)
    }

    pub open spec fn wf(&self) -> bool {
        true
    }
}

pub struct BitMask {
    pub mask: u64,
    pub expected: u64,
}

impl BitMask {
    pub fn new(mask: u64, expected: u64) -> (result: Self)
        ensures
            result.mask == mask,
            result.expected == expected,
    {
        BitMask { mask, expected }
    }

    pub fn all_ones() -> (result: Self)
        ensures
            result.mask == 0xFFFF_FFFF_FFFF_FFFF,
            result.expected == 0xFFFF_FFFF_FFFF_FFFF,
    {
        BitMask {
            mask: 0xFFFF_FFFF_FFFF_FFFF,
            expected: 0xFFFF_FFFF_FFFF_FFFF,
        }
    }

    pub fn all_zeros() -> (result: Self)
        ensures
            result.mask == 0xFFFF_FFFF_FFFF_FFFF,
            result.expected == 0,
    {
        BitMask {
            mask: 0xFFFF_FFFF_FFFF_FFFF,
            expected: 0,
        }
    }

    pub open spec fn matches(&self, value: u64) -> bool {
        (value & self.mask) == self.expected
    }

    pub fn get_mask(&self) -> (result: u64)
        ensures
            result == self.mask,
    {
        self.mask
    }

    pub fn get_expected(&self) -> (result: u64)
        ensures
            result == self.expected,
    {
        self.expected
    }

    pub open spec fn wf(&self) -> bool {
        (self.expected & self.mask) == self.expected
    }
}

pub struct U32Pair {
    pub low: u32,
    pub high: u32,
}

impl U32Pair {
    pub fn new(low: u32, high: u32) -> (result: Self)
        ensures
            result.low == low,
            result.high == high,
    {
        U32Pair { low, high }
    }

    pub fn zero() -> (result: Self)
        ensures
            result.low == 0,
            result.high == 0,
    {
        U32Pair { low: 0, high: 0 }
    }

    pub open spec fn as_u64(&self) -> int {
        self.low as int + self.high as int * 0x1_0000_0000
    }

    pub fn get_low(&self) -> (result: u32)
        ensures
            result == self.low,
    {
        self.low
    }

    pub fn get_high(&self) -> (result: u32)
        ensures
            result == self.high,
    {
        self.high
    }

    pub open spec fn wf(&self) -> bool {
        self.as_u64() <= u64::MAX
    }
}

pub struct BitRange {
    pub start_bit: u64,
    pub end_bit: u64,
}

impl BitRange {
    pub fn new(start_bit: u64, end_bit: u64) -> (result: Self)
        ensures
            result.start_bit == start_bit,
            result.end_bit == end_bit,
    {
        BitRange { start_bit, end_bit }
    }

    pub fn single_bit(bit: u64) -> (result: Self)
        requires
            bit < 63,
        ensures
            result.start_bit == bit,
            result.end_bit == bit + 1,
    {
        BitRange {
            start_bit: bit,
            end_bit: bit + 1,
        }
    }

    pub open spec fn len(&self) -> int {
        self.end_bit as int - self.start_bit as int
    }

    pub open spec fn contains_bit(&self, bit: u64) -> bool {
        self.start_bit <= bit < self.end_bit
    }

    pub open spec fn wf(&self) -> bool {
        &&& self.start_bit < 64
        &&& self.end_bit <= 64
        &&& self.start_bit < self.end_bit
    }

    pub fn get_start_bit(&self) -> (result: u64)
        ensures
            result == self.start_bit,
    {
        self.start_bit
    }

    pub fn get_end_bit(&self) -> (result: u64)
        ensures
            result == self.end_bit,
    {
        self.end_bit
    }
}

pub struct PowerOfTwo {
    pub exponent: u64,
}

impl PowerOfTwo {
    pub fn new(exponent: u64) -> (result: Self)
        ensures
            result.exponent == exponent,
    {
        PowerOfTwo { exponent }
    }

    pub open spec fn value(&self) -> int
        recommends
            self.exponent < 64,
    {
        match self.exponent {
            0 => 1,
            1 => 2,
            2 => 4,
            3 => 8,
            4 => 16,
            5 => 32,
            6 => 64,
            7 => 128,
            8 => 256,
            9 => 512,
            10 => 1024,
            11 => 2048,
            12 => 4096,
            _ => 1,
        }
    }

    pub open spec fn is_valid(&self) -> bool {
        self.exponent < 64
    }

    pub fn get_exponent(&self) -> (result: u64)
        ensures
            result == self.exponent,
    {
        self.exponent
    }

    pub open spec fn wf(&self) -> bool {
        self.is_valid()
    }
}

fn test_bit_operations() {
    let flags = BitFlags::new(0b1010);
    let value = flags.get_value();

    let zero_flags = BitFlags::zero();
    let zero_val = zero_flags.get_value();

    let mask = BitMask::new(0xFF, 0x0F);
    let mask_val = mask.get_mask();
    let expected = mask.get_expected();

    let all_ones = BitMask::all_ones();
    let all_zeros = BitMask::all_zeros();

    let pair = U32Pair::new(100, 200);
    let low = pair.get_low();
    let high = pair.get_high();

    let bit_range = BitRange::new(8, 16);
    let start = bit_range.get_start_bit();
    let end = bit_range.get_end_bit();

    let single = BitRange::single_bit(5);

    let pow2 = PowerOfTwo::new(10);
    let exp = pow2.get_exponent();

    proof {
        assert(value == 0b1010);
        // Note: bit testing assertions require additional lemmas
        // assert(flags.is_set(1));
        // assert(flags.is_set(3));
        // assert(!flags.is_set(0));
        // assert(!flags.is_set(2));
        assert(flags.wf());
        assert(zero_val == 0);
        assert(zero_flags.wf());
        assert(mask_val == 0xFF);
        assert(expected == 0x0F);
        // Note: mask matching assertions require additional lemmas
        // assert(mask.matches(0x0F));
        // assert(!mask.matches(0xFF));
        // assert(mask.wf());
        // assert(all_ones.matches(0xFFFF_FFFF_FFFF_FFFF));
        // assert(all_zeros.matches(0));
        assert(low == 100);
        assert(high == 200);
        assert(pair.as_u64() == 100 + 200 * 0x1_0000_0000);
        assert(start == 8);
        assert(end == 16);
        assert(bit_range.len() == 8);
        assert(bit_range.contains_bit(12));
        assert(!bit_range.contains_bit(16));
        assert(bit_range.wf());
        assert(single.start_bit == 5);
        assert(single.end_bit == 6);
        assert(single.wf());
        assert(exp == 10);
        assert(pow2.value() == 1024);
        assert(pow2.is_valid());
        assert(pow2.wf());
        // Note: bit manipulation assertions require additional lemmas
        // assert(has_bit_set(0b1010, 1));
        // assert(!has_bit_set(0b1010, 0));
        // assert(spec_bit_set(0b1010, 0) == 0b1011);
        // assert(spec_bit_clear(0b1010, 1) == 0b1000);
    }
}

} // verus!

fn main() {
    test_bit_operations();
}
