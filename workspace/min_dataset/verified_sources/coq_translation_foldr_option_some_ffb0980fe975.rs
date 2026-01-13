use vstd::prelude::*;

verus! {

pub open spec fn foldr_option<A, B>(o: Option<A>, init: B, f: spec_fn(A, B) -> B) -> B {
    match o {
        Option::None => init,
        Option::Some(a) => f(a, init),
    }
}


pub proof fn foldr_option_some<A, B>(a: A, init: B, f: spec_fn(A, B) -> B)
    ensures foldr_option(Option::Some(a), init, f) == f(a, init)
{
    // Trivially true
}

} // verus!