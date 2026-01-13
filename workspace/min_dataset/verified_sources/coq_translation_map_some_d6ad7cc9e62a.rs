use vstd::prelude::*;

verus! {

pub open spec fn map_option<T, U>(o: Option<T>, f: spec_fn(T) -> U) -> Option<U> {
    match o {
        Some(x) => Some(f(x)),
        None => None,
    }
}

pub open spec fn is_some<T>(o: Option<T>) -> bool {
    o.is_some()
}


pub proof fn map_some<T, U>(x: T, f: spec_fn(T) -> U)
    ensures is_some(map_option(Some(x), f))
{
}

} // verus!