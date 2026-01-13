use vstd::prelude::*;

verus! {

pub proof fn sub_def(a: int, b: int) ensures a - b == a + (-b) {}

} // verus!