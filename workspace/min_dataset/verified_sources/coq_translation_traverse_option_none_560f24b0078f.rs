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


pub proof fn traverse_option_none<A, B>(f: spec_fn(A) -> Option<B>)
    ensures traverse_option(Option::<A>::None, f) == Option::Some(Option::<B>::None)
{
    // Trivially true
}

} // verus!