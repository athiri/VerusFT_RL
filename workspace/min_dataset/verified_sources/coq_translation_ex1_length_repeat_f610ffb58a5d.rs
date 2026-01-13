use vstd::prelude::*;

verus! {

pub type NatList = Seq<nat>;

pub open spec fn length(xs: NatList) -> nat {
    xs.len()
}

pub open spec fn repeat(v: nat, count: nat) -> NatList {
    Seq::new(count, |i: int| v)
}


pub proof fn ex1_length_repeat(v: nat, n: nat)
    ensures length(repeat(v, n)) == n
{
    assert(repeat(v, n).len() == n);
}

} // verus!