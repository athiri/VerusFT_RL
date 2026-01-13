use vstd::prelude::*;

verus! {

pub open spec fn shrink_seq_tail<T>(s: Seq<T>) -> Seq<T> {
    if s.len() > 0 {
        s.take(s.len() - 1)
    } else {
        s
    }
}


pub proof fn verify_shrink_seq_tail_smaller<T>(s: Seq<T>)
    requires s.len() > 0
    ensures shrink_seq_tail(s).len() < s.len()
{
}

} // verus!