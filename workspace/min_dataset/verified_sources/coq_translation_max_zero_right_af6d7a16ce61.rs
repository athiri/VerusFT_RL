use vstd::prelude::*;

verus! {

pub open spec fn max(a: nat, b: nat) -> nat { if a > b { a } else { b } }


pub proof fn max_zero_right(a: nat) ensures max(a, 0) == a {}

} // verus!