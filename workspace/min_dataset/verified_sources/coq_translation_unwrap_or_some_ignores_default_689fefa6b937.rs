use vstd::prelude::*;

verus! {

pub open spec fn unwrap_or<A>(o: Option<A>, default: A) -> A {
    match o {
        Option::None => default,
        Option::Some(x) => x,
    }
}


pub proof fn unwrap_or_some_ignores_default<A>(x: A, default: A)
    ensures unwrap_or(Option::Some(x), default) == x
{
    // Trivially true
}

} // verus!