use vstd::prelude::*;

verus! {

pub open spec fn seq_last<T>(s: Seq<T>) -> T
    recommends s.len() > 0
{
    s[s.len() - 1]
}

pub open spec fn seq_snoc<T>(s: Seq<T>, x: T) -> Seq<T> {
    s.push(x)
}


pub proof fn snoc_last<T>(s: Seq<T>, x: T)
    ensures seq_last(seq_snoc(s, x)) == x
{
}

} // verus!