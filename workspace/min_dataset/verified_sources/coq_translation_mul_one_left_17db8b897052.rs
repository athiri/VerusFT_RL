use vstd::prelude::*;

verus! {

pub proof fn mul_one_left(a: nat) ensures 1 * a == a {}

} // verus!