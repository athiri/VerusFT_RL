use vstd::prelude::*;

verus! {

pub open spec fn meet(a: nat, b: nat) -> nat { if a < b { a } else { b } }


pub proof fn meet_idemp(a: nat) ensures meet(a, a) == a {}

} // verus!