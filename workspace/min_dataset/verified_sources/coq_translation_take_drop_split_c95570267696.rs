use vstd::prelude::*;

verus! {

pub open spec fn seq_append<T>(s1: Seq<T>, s2: Seq<T>) -> Seq<T> {
    s1 + s2
}

pub open spec fn seq_drop<T>(s: Seq<T>, n: nat) -> Seq<T> {
    s.skip(n as int)
}

pub open spec fn seq_take<T>(s: Seq<T>, n: nat) -> Seq<T> {
    s.take(n as int)
}


pub proof fn take_drop_split<T>(s: Seq<T>, n: nat)
    requires n <= s.len()
    ensures seq_append(seq_take(s, n), seq_drop(s, n)) =~= s
{
}

} // verus!