use vstd::prelude::*;

verus! {

pub open spec fn gen_nat_union(out1: Set<nat>, out2: Set<nat>) -> Set<nat> {
    out1.union(out2)
}


pub proof fn gen_nat_union_contains(out1: Set<nat>, out2: Set<nat>, n: nat)
    requires out1.contains(n) || out2.contains(n)
    ensures gen_nat_union(out1, out2).contains(n)
{
}

} // verus!