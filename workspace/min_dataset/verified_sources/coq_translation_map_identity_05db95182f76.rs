use vstd::prelude::*;

verus! {

pub open spec fn map_option<T, U>(o: Option<T>, f: spec_fn(T) -> U) -> Option<U> {
    match o {
        Some(x) => Some(f(x)),
        None => None,
    }
}


pub proof fn map_identity<T>(o: Option<T>)
    ensures map_option(o, |x: T| x) == o
{
}

} // verus!