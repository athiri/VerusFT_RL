use vstd::prelude::*;

verus! {

pub open spec fn seq_snoc<T>(s: Seq<T>, x: T) -> Seq<T> {
    s.push(x)
}


pub proof fn snoc_len<T>(s: Seq<T>, x: T)
    ensures seq_snoc(s, x).len() == s.len() + 1
{
}

} // verus!