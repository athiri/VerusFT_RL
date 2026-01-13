use vstd::prelude::*;

verus! {

pub open spec fn join(a: nat, b: nat) -> nat { if a > b { a } else { b } }

pub open spec fn meet(a: nat, b: nat) -> nat { if a < b { a } else { b } }


pub proof fn meet_distr_join(a: nat, b: nat, c: nat) ensures meet(a, join(b, c)) == join(meet(a, b), meet(a, c)) {}

} // verus!