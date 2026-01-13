// Source: verification/common/src/spec/common.rs (lines 21-27)
// Concept: A spec function defining a validity predicate used in an executable function's precondition
// Idea: Demonstrates how spec functions define domain constraints that executable code must satisfy
//
// The spec function `valid_cpu` checks if a CPU ID is within valid bounds.
// The executable function `get_cpu_offset` uses this spec in its requires clause
// to ensure the input is valid before computing an offset.

use vstd::prelude::*;

verus! {

pub open spec fn valid_cpu(cpu_num: nat, cpu: nat) -> bool {
    0 <= cpu < cpu_num
}

pub fn get_cpu_offset(cpu_num: usize, cpu: usize) -> (res: usize)
    requires
        valid_cpu(cpu_num as nat, cpu as nat),
        cpu * 64 < usize::MAX,
    ensures
        res == cpu * 64,
{
    cpu * 64
}

fn main() {}

} // verus!

