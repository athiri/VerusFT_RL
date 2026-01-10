// Source: verification/common/src/spec/common.rs (lines 33-35)
// Concept: A spec function for validating virtual addresses
// Idea: Demonstrates address space validation for virtual memory,
//       ensuring addresses are within the addressable range
//
// The spec function `valid_vaddr` checks if a virtual address is within
// the valid address space. The executable function `extract_page_offset`
// uses this to safely extract the page offset from an address.

use vstd::prelude::*;

verus! {

pub open spec fn valid_vaddr(va: usize, address_width: usize) -> bool {
    0 <= va < (1usize << address_width)
}

pub fn extract_page_offset(va: usize, address_width: usize, page_size: usize) -> (res: usize)
    requires
        valid_vaddr(va, address_width),
        page_size > 0,
        address_width <= 48,
    ensures
        res == va % page_size,
{
    va % page_size
}

fn main() {}

} // verus!

