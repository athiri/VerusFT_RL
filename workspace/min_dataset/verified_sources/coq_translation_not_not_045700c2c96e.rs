use vstd::prelude::*;

verus! {

pub open spec fn bool_not(a: bool) -> bool { !a }


pub proof fn not_not(a: bool) ensures bool_not(bool_not(a)) == a {}

} // verus!