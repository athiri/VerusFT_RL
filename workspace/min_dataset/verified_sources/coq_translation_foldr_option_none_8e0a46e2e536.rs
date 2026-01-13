use vstd::prelude::*;

verus! {

pub open spec fn foldr_option<A, B>(o: Option<A>, init: B, f: spec_fn(A, B) -> B) -> B {
    match o {
        Option::None => init,
        Option::Some(a) => f(a, init),
    }
}


pub proof fn foldr_option_none<A, B>(init: B, f: spec_fn(A, B) -> B)
    ensures foldr_option(Option::<A>::None, init, f) == init
{
    // Trivially true
}

} // verus!