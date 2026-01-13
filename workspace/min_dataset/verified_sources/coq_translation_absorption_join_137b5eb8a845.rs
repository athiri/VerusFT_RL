use vstd::prelude::*;

verus! {

pub open spec fn join(a: nat, b: nat) -> nat { if a > b { a } else { b } }

pub open spec fn meet(a: nat, b: nat) -> nat { if a < b { a } else { b } }


pub proof fn absorption_join(a: nat, b: nat) ensures join(a, meet(a, b)) == a {}

} // verus!