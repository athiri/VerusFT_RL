use vstd::prelude::*;

verus! {

pub open spec fn gen_choose<A>(gen1: Set<A>, gen2: Set<A>) -> Set<A> {
    gen1.union(gen2)
}


pub proof fn choose_right<A>(gen1: Set<A>, gen2: Set<A>, a: A)
    requires gen2.contains(a)
    ensures gen_choose(gen1, gen2).contains(a)
{
}

} // verus!