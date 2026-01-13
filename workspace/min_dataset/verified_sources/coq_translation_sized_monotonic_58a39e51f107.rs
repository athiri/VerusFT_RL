use vstd::prelude::*;

verus! {

pub open spec fn arbitrary_nat_range(min: nat, max: nat) -> Set<nat> {
    Set::new(|n: nat| n >= min && n <= max)
}


pub open spec fn arbitrary_nat_sized(size: nat) -> Set<nat> {
    arbitrary_nat_range(0, size)
}


pub proof fn sized_monotonic(size1: nat, size2: nat, n: nat)
    requires size1 <= size2, arbitrary_nat_sized(size1).contains(n)
    ensures arbitrary_nat_sized(size2).contains(n)
{
}

} // verus!