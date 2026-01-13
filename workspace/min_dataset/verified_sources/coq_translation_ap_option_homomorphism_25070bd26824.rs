use vstd::prelude::*;

verus! {

pub open spec fn pure_option<A>(a: A) -> Option<A> {
    Option::Some(a)
}

pub open spec fn ap_option<A, B>(
    of: Option<spec_fn(A) -> B>,
    oa: Option<A>
) -> Option<B> {
    match (of, oa) {
        (Option::Some(f), Option::Some(a)) => Option::Some(f(a)),
        _ => Option::None,
    }
}


pub proof fn ap_option_homomorphism<A, B>(f: spec_fn(A) -> B, x: A)
    ensures ap_option(pure_option(f), pure_option(x)) == pure_option(f(x))
{
    assert(ap_option(Option::Some(f), Option::Some(x)) == Option::Some(f(x)));
}

} // verus!