use vstd::prelude::*;

verus! {

pub open spec fn arbitrary_nat_range(min: nat, max: nat) -> Set<nat> {
    Set::new(|n: nat| n >= min && n <= max)
}


pub open spec fn arbitrary_small_nat() -> Set<nat> {
    arbitrary_nat_range(0, 100)
}

pub open spec fn arbitrary_large_nat() -> Set<nat> {
    arbitrary_nat_range(0, 1000000)
}


pub proof fn small_in_large()
    ensures forall|n: nat| arbitrary_small_nat().contains(n) ==> arbitrary_large_nat().contains(n)
{
}

} // verus!