use vstd::prelude::*;

verus! {

pub open spec fn return_outputs<T>(x: T) -> Set<T> {
    set![x]
}


pub proof fn return_only<T>(x: T, y: T)
    requires return_outputs(x).contains(y)
    ensures x == y
{
}

} // verus!