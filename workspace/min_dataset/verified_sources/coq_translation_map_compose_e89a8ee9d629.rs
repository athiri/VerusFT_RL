use vstd::prelude::*;

verus! {

pub open spec fn map_option<T, U>(o: Option<T>, f: spec_fn(T) -> U) -> Option<U> {
    match o {
        Some(x) => Some(f(x)),
        None => None,
    }
}


pub proof fn map_compose<T, U, V>(o: Option<T>, f: spec_fn(T) -> U, g: spec_fn(U) -> V)
    ensures map_option(map_option(o, f), g) == map_option(o, |x: T| g(f(x)))
{
}

} // verus!