use vstd::prelude::*;

verus! {

pub open spec fn abs(x: int) -> nat { if x >= 0 { x as nat } else { (-x) as nat } }


pub proof fn abs_nonneg(x: int) ensures abs(x) >= 0 {}

} // verus!