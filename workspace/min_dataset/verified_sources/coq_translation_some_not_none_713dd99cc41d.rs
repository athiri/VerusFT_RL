use vstd::prelude::*;

verus! {

pub open spec fn is_some<T>(o: Option<T>) -> bool {
    o.is_some()
}

pub open spec fn is_none<T>(o: Option<T>) -> bool {
    o.is_none()
}


pub proof fn some_not_none<T>(x: T)
    ensures is_some(Some(x)), !is_none(Some(x))
{
}

} // verus!