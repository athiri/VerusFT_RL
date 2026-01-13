use vstd::prelude::*;

verus! {

pub struct FenwickTree { pub tree: Seq<nat>, pub n: nat }


pub open spec fn prefix_sum(ft: FenwickTree, i: nat) -> nat
    decreases i
{
    if i == 0 { 0 }
    else if i > ft.n { 0 }
    else { ft.tree[(i - 1) as int] + prefix_sum(ft, (i - 1) as nat) }
}

} // verus!