use vstd::prelude::*;

verus! {

pub open spec fn and_then<T, U>(o: Option<T>, f: spec_fn(T) -> Option<U>) -> Option<U> {
    match o {
        Some(x) => f(x),
        None => None,
    }
}


pub proof fn and_then_return<T, U>(x: T, f: spec_fn(T) -> Option<U>)
    ensures and_then(Some(x), f) == f(x)
{
}

} // verus!