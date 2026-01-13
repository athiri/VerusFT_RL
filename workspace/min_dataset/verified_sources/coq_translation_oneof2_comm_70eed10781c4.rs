use vstd::prelude::*;

verus! {

pub open spec fn oneof2<T>(gen1: Set<T>, gen2: Set<T>) -> Set<T> {
    gen1.union(gen2)
}


pub proof fn oneof2_comm<T>(gen1: Set<T>, gen2: Set<T>)
    ensures oneof2(gen1, gen2) =~= oneof2(gen2, gen1)
{
    // Union is commutative
}

} // verus!