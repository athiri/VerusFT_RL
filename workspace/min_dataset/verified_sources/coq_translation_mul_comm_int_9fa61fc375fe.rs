use vstd::prelude::*;

verus! {

pub proof fn mul_comm_int(a: int, b: int) ensures a * b == b * a {}

} // verus!