use vstd::prelude::*;

verus! {

pub proof fn add_comm_int(a: int, b: int) ensures a + b == b + a {}

} // verus!