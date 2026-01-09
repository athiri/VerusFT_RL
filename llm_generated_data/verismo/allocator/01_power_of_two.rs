// Variation: Power of Two Operations
// From: source/verismo/src/allocator/bit_p.rs, buddy.rs
// Demonstrates: Power-of-2 checking, alignment verification

use vstd::prelude::*;

verus! {

pub open spec fn spec_is_power_of_two(n: u64) -> bool {
    n > 0 && (n & (n - 1) as u64) == 0
}

pub fn is_power_of_two(n: u64) -> (result: bool)
    ensures
        result <==> spec_is_power_of_two(n),
{
    n > 0 && (n & (n - 1)) == 0
}

pub fn is_aligned_to_power_of_two(addr: u64, align: u64) -> (result: bool)
    requires
        spec_is_power_of_two(align),
    ensures
        result <==> (addr & (align - 1) as u64) == 0,
{
    (addr & (align - 1)) == 0
}

pub fn check_alignment(addr: u64, align: u64) -> (result: bool)
    requires
        spec_is_power_of_two(align),
    ensures
        result <==> addr % align == 0,
{
    addr % align == 0
}

pub fn get_mask(align: u64) -> (result: u64)
    requires
        spec_is_power_of_two(align),
    ensures
        result == align - 1,
{
    align - 1
}

pub fn is_aligned_bitwise(addr: u64, align: u64) -> (result: bool)
    requires
        spec_is_power_of_two(align),
    ensures
        result <==> (addr & (align - 1) as u64) == 0,
{
    let mask = get_mask(align);
    (addr & mask) == 0
}

proof fn lemma_is_power_of_two_16()
    ensures spec_is_power_of_two(16),
{
    assert(16u64 > 0) by(compute_only);
    assert((16u64 & 15u64) == 0) by(compute_only);
}

proof fn lemma_is_power_of_two_8()
    ensures spec_is_power_of_two(8),
{
    assert(8u64 > 0) by(compute_only);
    assert((8u64 & 7u64) == 0) by(compute_only);
}

proof fn lemma_is_power_of_two_256()
    ensures spec_is_power_of_two(256),
{
    assert(256u64 > 0) by(compute_only);
    assert((256u64 & 255u64) == 0) by(compute_only);
}

fn test_power_of_two() {
    let pow2_check_1 = is_power_of_two(8);
    let pow2_check_2 = is_power_of_two(7);
    let pow2_check_3 = is_power_of_two(16);
    let pow2_check_4 = is_power_of_two(1);

    proof {
        lemma_is_power_of_two_16();
        lemma_is_power_of_two_8();
        lemma_is_power_of_two_256();
    }

    let aligned_1 = is_aligned_to_power_of_two(0x1000, 16);
    let aligned_2 = is_aligned_to_power_of_two(0x1001, 16);
    let aligned_3 = is_aligned_to_power_of_two(0x2000, 8);

    let check1 = check_alignment(0x2000, 16);
    let check2 = check_alignment(0x2001, 16);
    let check3 = check_alignment(0x1000, 256);

    let mask1 = get_mask(16);
    let mask2 = get_mask(256);

    let bitwise1 = is_aligned_bitwise(0x1000, 16);
    let bitwise2 = is_aligned_bitwise(0x1001, 16);
}

} // verus!

fn main() {
    test_power_of_two();
}
