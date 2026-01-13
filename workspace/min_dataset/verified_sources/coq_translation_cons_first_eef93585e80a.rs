use vstd::prelude::*;

verus! {

pub open spec fn seq_first<T>(s: Seq<T>) -> T
    recommends s.len() > 0
{
    s[0]
}

pub open spec fn seq_cons<T>(x: T, s: Seq<T>) -> Seq<T> {
    seq![x] + s
}


pub proof fn cons_first<T>(x: T, s: Seq<T>)
    ensures seq_first(seq_cons(x, s)) == x
{
}

} // verus!