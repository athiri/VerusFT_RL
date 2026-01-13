use vstd::prelude::*;

verus! {

pub open spec fn join(a: nat, b: nat) -> nat { if a > b { a } else { b } }


pub proof fn join_idemp(a: nat) ensures join(a, a) == a {}

} // verus!