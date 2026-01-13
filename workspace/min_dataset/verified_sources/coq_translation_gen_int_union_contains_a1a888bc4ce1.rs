use vstd::prelude::*;

verus! {

pub open spec fn gen_int_union(out1: Set<int>, out2: Set<int>) -> Set<int> {
    out1.union(out2)
}


pub proof fn gen_int_union_contains(out1: Set<int>, out2: Set<int>, n: int)
    requires out1.contains(n) || out2.contains(n)
    ensures gen_int_union(out1, out2).contains(n)
{
}

} // verus!