use vstd::prelude::*;

verus! {

pub open spec fn map_option<T, U>(o: Option<T>, f: spec_fn(T) -> U) -> Option<U> {
    match o {
        Some(x) => Some(f(x)),
        None => None,
    }
}

pub open spec fn is_none<T>(o: Option<T>) -> bool {
    o.is_none()
}


pub proof fn map_none<T, U>(f: spec_fn(T) -> U)
    ensures is_none(map_option::<T, U>(None, f))
{
}

} // verus!