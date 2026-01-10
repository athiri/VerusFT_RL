// Source: verification/vstd_extra/src/set_extra.rs (lines 147-156)
// Concept: A spec function for checking if a midpoint partitions a range
// Idea: Demonstrates partition properties useful for divide-and-conquer,
//       ensuring the midpoint is within valid bounds
//
// The spec function `valid_partition_point` checks if mid partitions [0, len).
// The executable function `get_partition_sizes` computes the sizes of
// the two partitions.

use vstd::prelude::*;

verus! {

pub open spec fn valid_partition_point(mid: nat, len: nat) -> bool {
    mid <= len
}

pub fn get_partition_sizes(len: usize, mid: usize) -> (res: (usize, usize))
    requires
        valid_partition_point(mid as nat, len as nat),
    ensures
        res.0 == mid,
        res.1 == len - mid,
        res.0 + res.1 == len,
{
    (mid, len - mid)
}

fn main() {}

} // verus!
