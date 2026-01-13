// Source: verification/common/src/spec/common.rs (lines 29-31)
// Concept: A spec function for validating page table entry offsets
// Idea: Demonstrates domain validation for page table indexing,
//       ensuring offsets stay within valid bounds for memory management
//
// The spec function `valid_pte_offset` checks if an offset is within
// the valid range for page table entries. The executable function
// `compute_pte_address` uses this to safely compute entry addresses.

use vstd::prelude::*;

verus! {

pub open spec fn valid_pte_offset(offset: nat, nr_entries: nat) -> bool {
    0 <= offset < nr_entries
}

pub fn compute_pte_address(base: usize, offset: usize, nr_entries: usize, entry_size: usize) -> (res: usize)
    requires
        valid_pte_offset(offset as nat, nr_entries as nat),
        entry_size > 0,
        base + offset * entry_size < usize::MAX,
    ensures
        res == base + offset * entry_size,
{
    base + offset * entry_size
}

fn main() {}

} // verus!

