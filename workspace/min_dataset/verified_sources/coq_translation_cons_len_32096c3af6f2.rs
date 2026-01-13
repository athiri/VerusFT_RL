use vstd::prelude::*;

verus! {

pub open spec fn seq_cons<T>(x: T, s: Seq<T>) -> Seq<T> {
    seq![x] + s
}


pub proof fn cons_len<T>(x: T, s: Seq<T>)
    ensures seq_cons(x, s).len() == s.len() + 1
{
}

} // verus!