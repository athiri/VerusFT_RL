use vstd::prelude::*;

verus! {

pub open spec fn option_fmap<A, B>(f: spec_fn(A) -> B, m: Option<A>) -> Option<B> {
    match m {
        None => None,
        Some(a) => Some(f(a)),
    }
}


pub proof fn option_fmap_identity<A>(m: Option<A>)
    ensures option_fmap(|x: A| x, m) == m
{
}

} // verus!