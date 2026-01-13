// Variation: Page Alignment Operations
// From: source/verismo/src/addr_e/exe.rs
// Demonstrates: Address alignment, modulo arithmetic verification

use vstd::prelude::*;

verus! {

pub const PAGE_SIZE: usize = 4096;

pub open spec fn spec_align_up(value: int, alignment: int) -> int {
    let remainder = value % alignment;
    if remainder == 0 {
        value
    } else {
        value + (alignment - remainder)
    }
}

pub open spec fn spec_align_down(value: int, alignment: int) -> int {
    value - (value % alignment)
}

pub fn page_align_up(value: usize) -> (ret: usize)
    requires
        value <= usize::MAX - PAGE_SIZE,
    ensures
        ret as int % PAGE_SIZE as int == 0,
        ret == spec_align_up(value as int, PAGE_SIZE as int),
        value as int <= ret as int,
        (ret as int) < (value as int) + PAGE_SIZE as int,
{
    let remainder = value % PAGE_SIZE;
    if remainder == 0 {
        value
    } else {
        value + (PAGE_SIZE - remainder)
    }
}

pub fn page_align_down(value: usize) -> (ret: usize)
    ensures
        ret as int % PAGE_SIZE as int == 0,
        ret == spec_align_down(value as int, PAGE_SIZE as int),
        (value as int) - PAGE_SIZE as int <= ret as int,
        ret as int <= value as int,
{
    value - (value % PAGE_SIZE)
}

pub fn is_page_aligned(value: usize) -> (result: bool)
    ensures
        result <==> value as int % PAGE_SIZE as int == 0,
{
    value % PAGE_SIZE == 0
}

fn test_alignment() {
    let addr1 = 4095usize;
    let aligned1 = page_align_up(addr1);
    assert(aligned1 == 4096usize);

    let check1 = is_page_aligned(aligned1);
    assert(check1);

    let addr2 = 4097usize;
    let aligned2 = page_align_down(addr2);
    assert(aligned2 == 4096usize);

    let check2 = is_page_aligned(aligned2);
    assert(check2);

    let addr3 = 8192usize;
    let aligned3 = page_align_up(addr3);
    assert(aligned3 == 8192usize);
}

} // verus!

fn main() {
    test_alignment();
}
