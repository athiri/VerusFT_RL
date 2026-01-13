use vstd::prelude::*;

verus! {

pub open spec fn meet(a: nat, b: nat) -> nat { if a < b { a } else { b } }


pub proof fn meet_assoc(a: nat, b: nat, c: nat) ensures meet(meet(a, b), c) == meet(a, meet(b, c)) {}

} // verus!