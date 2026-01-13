use vstd::prelude::*;

verus! {

pub open spec fn is_fixpoint(f: spec_fn(nat) -> nat, x: nat) -> bool { f(x) == x }


pub proof fn zero_fixpoint_id() ensures is_fixpoint(|x: nat| x, 0) {}

} // verus!