use vstd::prelude::*;

verus! {

pub proof fn mul_one_right(a: nat) ensures a * 1 == a {}

} // verus!