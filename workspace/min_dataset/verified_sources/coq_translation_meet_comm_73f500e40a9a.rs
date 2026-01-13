use vstd::prelude::*;

verus! {

pub open spec fn meet(a: nat, b: nat) -> nat { if a < b { a } else { b } }


pub proof fn meet_comm(a: nat, b: nat) ensures meet(a, b) == meet(b, a) {}

} // verus!