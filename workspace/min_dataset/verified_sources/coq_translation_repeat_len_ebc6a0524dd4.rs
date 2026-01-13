use vstd::prelude::*;

verus! {

pub open spec fn seq_repeat<T>(x: T, n: nat) -> Seq<T> {
    Seq::new(n, |i: int| x)
}


pub proof fn repeat_len<T>(x: T, n: nat)
    ensures seq_repeat(x, n).len() == n
{
}

} // verus!