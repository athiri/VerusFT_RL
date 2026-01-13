use vstd::prelude::*;

verus! {

pub proof fn add_neg(a: int) ensures a + (-a) == 0 {}

} // verus!