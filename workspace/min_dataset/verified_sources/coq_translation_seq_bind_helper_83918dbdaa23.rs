use vstd::prelude::*;

verus! {

pub open spec fn seq_bind_helper<A, B>(s: Seq<A>, f: spec_fn(A) -> Seq<B>, idx: int) -> Seq<B>
    decreases s.len() - idx
{
    if idx >= s.len() {
        Seq::empty()
    } else {
        f(s[idx]) + seq_bind_helper(s, f, idx + 1)
    }
}

} // verus!