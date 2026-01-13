// Variation: Register Bit Fields
// From: source/verismo/src/registers/ patterns
// Demonstrates: Bit field extraction and validation

use vstd::prelude::*;

verus! {

// Simulated register with bit fields
pub struct Register {
    pub raw: u32,
}

impl Register {
    pub fn new() -> (result: Self)
        ensures
            result.raw == 0,
    {
        Register { raw: 0 }
    }

    pub fn from_raw(value: u32) -> (result: Self)
        ensures
            result.raw == value,
    {
        Register { raw: value }
    }

    pub fn get_raw(&self) -> (result: u32)
        ensures
            result == self.raw,
    {
        self.raw
    }

    pub fn set_raw(&mut self, value: u32)
        ensures
            self.raw == value,
    {
        self.raw = value;
    }

    pub fn is_bit_set(&self, bit: u32) -> (result: bool)
        requires
            bit < 32,
        ensures
            result <==> (self.raw & (1u32 << bit)) != 0,
    {
        (self.raw & (1u32 << bit)) != 0
    }

    pub fn set_bit(&mut self, bit: u32)
        requires
            bit < 32,
        ensures
            self.raw == old(self).raw | (1u32 << bit),
    {
        self.raw = self.raw | (1u32 << bit);
    }

    pub fn clear_bit(&mut self, bit: u32)
        requires
            bit < 32,
        ensures
            self.raw == old(self).raw & !(1u32 << bit),
    {
        self.raw = self.raw & !(1u32 << bit);
    }

    pub fn toggle_bit(&mut self, bit: u32)
        requires
            bit < 32,
        ensures
            self.raw == old(self).raw ^ (1u32 << bit),
    {
        self.raw = self.raw ^ (1u32 << bit);
    }

    pub fn count_set_bits(&self) -> (result: u8)
        ensures
            result <= 32,
    {
        let mut count = 0;
        let mut v = self.raw;
        let mut i = 0;
        while i < 32
            invariant
                0 <= i <= 32,
                count <= i,
            decreases 32 - i
        {
            if v & 1 == 1 {
                count = count + 1;
            }
            v = v >> 1;
            i = i + 1;
        }
        count
    }
}

fn test_register() {
    let mut reg = Register::new();
    assert(reg.raw == 0);

    reg.set_bit(0);
    reg.set_bit(5);
    reg.set_bit(10);

    let count = reg.count_set_bits();
    assert(count <= 32);

    let raw1 = reg.get_raw();

    reg.toggle_bit(5);
    let raw2 = reg.get_raw();
    assert(raw2 == raw1 ^ (1u32 << 5));

    reg.clear_bit(10);
    let raw3 = reg.get_raw();
    assert(raw3 == raw2 & !(1u32 << 10));
}

} // verus!

fn main() {
    test_register();
}
