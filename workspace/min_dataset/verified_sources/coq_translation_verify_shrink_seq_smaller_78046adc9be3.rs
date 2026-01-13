use vstd::prelude::*;

verus! {

pub open spec fn shrink_seq_head<T>(s: Seq<T>) -> Seq<T> {
    if s.len() > 0 {
        s.skip(1)
    } else {
        s
    }
}


pub proof fn verify_shrink_seq_smaller<T>(s: Seq<T>)
    requires s.len() > 0
    ensures shrink_seq_head(s).len() < s.len()
{
}

} // verus!