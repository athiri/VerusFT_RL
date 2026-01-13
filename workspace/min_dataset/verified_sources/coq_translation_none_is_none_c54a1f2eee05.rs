use vstd::prelude::*;

verus! {

pub open spec fn is_some<T>(o: Option<T>) -> bool {
    o.is_some()
}

pub open spec fn is_none<T>(o: Option<T>) -> bool {
    o.is_none()
}


pub proof fn none_is_none<T>()
    ensures is_none::<T>(None), !is_some::<T>(None)
{
}

} // verus!