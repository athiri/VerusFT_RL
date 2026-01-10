// Source: verification/common/src/exec.rs (lines 283-285)
// Concept: A spec function for computing an index from an address
// Idea: Demonstrates address-to-index conversion in memory management,
//       useful for frame table lookups
//
// The spec function `addr_to_index_spec` computes the frame index from an address.
// The executable function `addr_to_index` performs the same computation and
// verifies it matches the spec.

use vstd::prelude::*;

verus! {

pub open spec fn addr_to_index_spec(addr: usize, base: usize, size: usize) -> usize {
    ((addr - base) / size as int) as usize
}

pub fn addr_to_index(addr: usize, base: usize, size: usize) -> (res: usize)
    requires
        addr >= base,
        size > 0,
    ensures
        res == addr_to_index_spec(addr, base, size),
{
    (addr - base) / size
}

fn main() {}

} // verus!
