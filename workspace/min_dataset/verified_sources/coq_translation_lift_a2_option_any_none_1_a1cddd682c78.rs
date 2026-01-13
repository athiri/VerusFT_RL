use vstd::prelude::*;

verus! {

pub open spec fn lift_a2_option<A, B, C>(
    f: spec_fn(A, B) -> C,
    oa: Option<A>,
    ob: Option<B>
) -> Option<C> {
    match (oa, ob) {
        (Option::Some(a), Option::Some(b)) => Option::Some(f(a, b)),
        _ => Option::None,
    }
}


pub proof fn lift_a2_option_any_none_1<A, B, C>(f: spec_fn(A, B) -> C, ob: Option<B>)
    ensures lift_a2_option(f, Option::<A>::None, ob) == Option::<C>::None
{
    // Trivially true
}

} // verus!