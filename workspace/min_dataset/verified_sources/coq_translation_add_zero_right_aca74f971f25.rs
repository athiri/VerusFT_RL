use vstd::prelude::*;

verus! {

pub proof fn add_zero_right(a: nat) ensures a + 0 == a {}

} // verus!