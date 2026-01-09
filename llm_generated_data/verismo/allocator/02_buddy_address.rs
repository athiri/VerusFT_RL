// Variation: Buddy Address Calculation
// From: source/verismo/src/allocator/bit_p.rs (proof_buddy)
// Demonstrates: Finding buddy addresses in buddy allocator

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

pub fn calculate_buddy(addr: u64, size: u64) -> (result: u64)
    requires
        size > 0,
        spec_is_power_of_two(size),
        addr % size == 0,
    ensures
        result == addr ^ size,
{
    addr ^ size
}

pub fn are_buddies(addr1: u64, addr2: u64, size: u64) -> (result: bool)
    requires
        size > 0,
        spec_is_power_of_two(size),
    ensures
        result <==> (addr1 ^ addr2 == size),
{
    addr1 ^ addr2 == size
}

pub fn can_merge_buddies(addr1: u64, addr2: u64, size: u64) -> (result: bool)
    requires
        size > 0,
        spec_is_power_of_two(size),
        addr1 % size == 0,
        addr2 % size == 0,
    ensures
        result <==> (addr1 ^ addr2 == size),
{
    are_buddies(addr1, addr2, size)
}

pub fn get_lower_address(addr1: u64, addr2: u64) -> (result: u64)
    ensures
        result == if addr1 < addr2 { addr1 } else { addr2 },
{
    if addr1 < addr2 { addr1 } else { addr2 }
}

pub fn get_higher_address(addr1: u64, addr2: u64) -> (result: u64)
    ensures
        result == if addr1 > addr2 { addr1 } else { addr2 },
{
    if addr1 > addr2 { addr1 } else { addr2 }
}

pub fn split_block_first(addr: u64) -> (result: u64)
    ensures
        result == addr,
{
    addr
}

pub fn split_block_second(addr: u64, size: u64) -> (result: u64)
    requires
        size >= 2,
        addr <= u64::MAX - size / 2,
    ensures
        result == addr + size / 2,
{
    addr + size / 2
}

proof fn lemma_is_power_of_two_0x1000()
    ensures spec_is_power_of_two(0x1000),
{
    assert(0x1000u64 > 0) by(compute_only);
    assert((0x1000u64 & 0x0FFFu64) == 0) by(compute_only);
}

fn test_buddy_address() {
    let addr = 0x1000u64;
    let size = 0x1000u64;

    proof {
        lemma_is_power_of_two_0x1000();
    }

    let buddy = calculate_buddy(addr, size);

    let addr1 = 0x2000u64;
    let addr2 = 0x3000u64;
    let buddies = are_buddies(addr1, addr2, 0x1000);

    let can_merge = can_merge_buddies(addr1, addr2, 0x1000);

    let lower = get_lower_address(addr1, addr2);
    let higher = get_higher_address(addr1, addr2);

    let first = split_block_first(0x4000);
    let second = split_block_second(0x4000, 0x2000);
}

} // verus!

fn main() {
    test_buddy_address();
}
