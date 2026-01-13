use vstd::prelude::*;

verus! {

pub open spec fn option_fmap<A, B>(f: spec_fn(A) -> B, m: Option<A>) -> Option<B> {
    match m {
        None => None,
        Some(a) => Some(f(a)),
    }
}


pub proof fn option_fmap_composition<A, B, C>(
    f: spec_fn(B) -> C,
    g: spec_fn(A) -> B,
    m: Option<A>
)
    ensures option_fmap(|x: A| f(g(x)), m) == option_fmap(f, option_fmap(g, m))
{
}

} // verus!