use vstd::prelude::*;

verus! {

pub open spec fn gen_nat_sized(size: nat) -> Set<nat> {
    Set::new(|n: nat| n <= size)
}


pub proof fn gen_nat_sized_monotonic(size1: nat, size2: nat, n: nat)
    requires size1 <= size2, gen_nat_sized(size1).contains(n)
    ensures gen_nat_sized(size2).contains(n)
{
}

} // verus!