use vstd::prelude::*;

verus! {

pub open spec fn seq_append<T>(s1: Seq<T>, s2: Seq<T>) -> Seq<T> {
    s1 + s2
}


pub proof fn append_len<T>(s1: Seq<T>, s2: Seq<T>)
    ensures seq_append(s1, s2).len() == s1.len() + s2.len()
{
}

} // verus!