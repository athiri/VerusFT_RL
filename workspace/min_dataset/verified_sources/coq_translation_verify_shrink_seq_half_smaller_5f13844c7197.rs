use vstd::prelude::*;

verus! {

pub open spec fn shrink_seq_half<T>(s: Seq<T>) -> Seq<T> {
    s.take((s.len() / 2) as int)
}


pub proof fn verify_shrink_seq_half_smaller<T>(s: Seq<T>)
    requires s.len() > 1
    ensures shrink_seq_half(s).len() < s.len()
{
}

} // verus!