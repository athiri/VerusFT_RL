use vstd::prelude::*;

verus! {

pub proof fn add_assoc_int(a: int, b: int, c: int) ensures (a + b) + c == a + (b + c) {}

} // verus!