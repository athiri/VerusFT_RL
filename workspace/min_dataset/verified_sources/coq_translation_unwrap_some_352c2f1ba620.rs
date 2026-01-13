use vstd::prelude::*;

verus! {

pub open spec fn unwrap_or<T>(o: Option<T>, default: T) -> T {
    match o {
        Some(x) => x,
        None => default,
    }
}


pub proof fn unwrap_some<T>(x: T, default: T)
    ensures unwrap_or(Some(x), default) == x
{
}

} // verus!