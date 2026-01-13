use vstd::prelude::*;

verus! {

pub open spec fn gen_nat_sized(size: nat) -> Set<nat> {
    Set::new(|n: nat| n <= size)
}


pub proof fn gen_nat_sized_bounded(size: nat, n: nat)
    requires gen_nat_sized(size).contains(n)
    ensures n <= size
{
}

} // verus!