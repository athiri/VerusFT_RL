use vstd::prelude::*;

verus! {

pub open spec fn cmp_nat(a: nat, b: nat) -> Ordering {
    if a < b { Ordering::Lt }
    else if a == b { Ordering::Eq }
    else { Ordering::Gt }
}


pub enum Ordering {
    Lt,
    Eq,
    Gt,
}


pub open spec fn cmp_seq_helper(s1: Seq<nat>, s2: Seq<nat>, idx: int) -> Ordering
    decreases s1.len() - idx
{
    if idx >= s1.len() && idx >= s2.len() {
        Ordering::Eq
    } else if idx >= s1.len() {
        Ordering::Lt
    } else if idx >= s2.len() {
        Ordering::Gt
    } else {
        match cmp_nat(s1[idx], s2[idx]) {
            Ordering::Lt => Ordering::Lt,
            Ordering::Gt => Ordering::Gt,
            Ordering::Eq => cmp_seq_helper(s1, s2, idx + 1),
        }
    }
}

} // verus!