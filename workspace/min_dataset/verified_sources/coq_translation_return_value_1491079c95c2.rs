use vstd::prelude::*;

verus! {

pub open spec fn return_outputs<T>(x: T) -> Set<T> {
    set![x]
}


pub proof fn return_value<T>(x: T)
    ensures return_outputs(x).contains(x)
{
}

} // verus!