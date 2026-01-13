use vstd::prelude::*;

verus! {

pub open spec fn ap_option<A, B>(
    of: Option<spec_fn(A) -> B>,
    oa: Option<A>
) -> Option<B> {
    match (of, oa) {
        (Option::Some(f), Option::Some(a)) => Option::Some(f(a)),
        _ => Option::None,
    }
}


pub proof fn ap_option_identity<A>(v: Option<A>)
    ensures ap_option(Option::Some(|a: A| a), v) == v
{
    match v {
        Option::None => {
            assert(ap_option(Option::Some(|a: A| a), Option::<A>::None) == Option::<A>::None);
        }
        Option::Some(x) => {
            assert(ap_option(Option::Some(|a: A| a), Option::Some(x)) == Option::Some(x));
        }
    }
}

} // verus!