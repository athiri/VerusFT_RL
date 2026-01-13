use vstd::prelude::*;

verus! {

pub open spec fn gen_choose<A>(gen1: Set<A>, gen2: Set<A>) -> Set<A> {
    gen1.union(gen2)
}


pub proof fn choose_commutative<A>(gen1: Set<A>, gen2: Set<A>)
    ensures gen_choose(gen1, gen2) =~= gen_choose(gen2, gen1)
{
}

} // verus!