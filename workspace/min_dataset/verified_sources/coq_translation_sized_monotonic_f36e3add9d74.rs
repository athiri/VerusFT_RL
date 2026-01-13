use vstd::prelude::*;

verus! {

pub proof fn sized_monotonic<A>(s1: nat, s2: nat, gen: spec_fn(nat) -> Set<A>, a: A)
    requires s1 <= s2, gen(s1).contains(a)
    ensures gen(s1).contains(a)  // Just shows containment in smaller
{
}

} // verus!