use vstd::prelude::*;

verus! {

pub open spec fn unwrap_or<A>(o: Option<A>, default: A) -> A {
    match o {
        Option::None => default,
        Option::Some(x) => x,
    }
}


pub proof fn unwrap_or_none_is_default<A>(default: A)
    ensures unwrap_or(Option::None, default) == default
{
    // Trivially true
}

} // verus!