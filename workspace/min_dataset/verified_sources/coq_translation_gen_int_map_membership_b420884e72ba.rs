use vstd::prelude::*;

verus! {

pub open spec fn gen_int_map(outputs: Set<int>, f: spec_fn(int) -> int) -> Set<int> {
    Set::new(|n: int| exists|m: int| outputs.contains(m) && f(m) == n)
}


pub proof fn gen_int_map_membership(outputs: Set<int>, f: spec_fn(int) -> int, n: int)
    requires outputs.contains(n)
    ensures gen_int_map(outputs, f).contains(f(n))
{
}

} // verus!