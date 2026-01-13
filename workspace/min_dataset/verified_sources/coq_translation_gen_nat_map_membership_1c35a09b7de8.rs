use vstd::prelude::*;

verus! {

pub open spec fn gen_nat_map(outputs: Set<nat>, f: spec_fn(nat) -> nat) -> Set<nat> {
    Set::new(|n: nat| exists|m: nat| outputs.contains(m) && f(m) == n)
}


pub proof fn gen_nat_map_membership(outputs: Set<nat>, f: spec_fn(nat) -> nat, n: nat)
    requires outputs.contains(n)
    ensures gen_nat_map(outputs, f).contains(f(n))
{
}

} // verus!