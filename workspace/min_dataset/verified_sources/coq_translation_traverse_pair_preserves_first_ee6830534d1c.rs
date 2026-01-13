use vstd::prelude::*;

verus! {

pub type Key = nat;

pub type M = Map<Key, int>;

pub open spec fn traverse_pair<M, A, B>(
    p: (M, A),
    f: spec_fn(A) -> Option<B>
) -> Option<(M, B)> {
    match f(p.1) {
        Option::None => Option::None,
        Option::Some(b) => Option::Some((p.0, b)),
    }
}


pub proof fn traverse_pair_preserves_first<M, A, B>(p: (M, A), f: spec_fn(A) -> Option<B>, b: B)
    requires f(p.1) == Option::Some(b)
    ensures traverse_pair(p, f) == Option::Some((p.0, b))
{
    // Trivially true
}

} // verus!