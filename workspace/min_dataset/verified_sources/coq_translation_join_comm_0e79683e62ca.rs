use vstd::prelude::*;

verus! {

pub open spec fn join(a: nat, b: nat) -> nat { if a > b { a } else { b } }


pub proof fn join_comm(a: nat, b: nat) ensures join(a, b) == join(b, a) {}

} // verus!