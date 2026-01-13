use vstd::prelude::*;

verus! {

pub open spec fn traverse_option<A, B>(o: Option<A>, f: spec_fn(A) -> Option<B>) -> Option<Option<B>> {
    match o {
        Option::None => Option::Some(Option::None),
        Option::Some(a) => match f(a) {
            Option::None => Option::None,
            Option::Some(b) => Option::Some(Option::Some(b)),
        }
    }
}


pub proof fn traverse_option_some_some<A, B>(a: A, b: B, f: spec_fn(A) -> Option<B>)
    requires f(a) == Option::Some(b)
    ensures traverse_option(Option::Some(a), f) == Option::Some(Option::Some(b))
{
    // Trivially true
}

} // verus!