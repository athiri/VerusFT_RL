use vstd::prelude::*;

verus! {

pub open spec fn bool_xor(a: bool, b: bool) -> bool { a != b }


pub proof fn xor_comm(a: bool, b: bool) ensures bool_xor(a, b) == bool_xor(b, a) {}

} // verus!