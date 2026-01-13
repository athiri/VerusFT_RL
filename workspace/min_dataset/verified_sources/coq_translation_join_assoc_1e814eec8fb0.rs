use vstd::prelude::*;

verus! {

pub open spec fn join(a: nat, b: nat) -> nat { if a > b { a } else { b } }


pub proof fn join_assoc(a: nat, b: nat, c: nat) ensures join(join(a, b), c) == join(a, join(b, c)) {}

} // verus!