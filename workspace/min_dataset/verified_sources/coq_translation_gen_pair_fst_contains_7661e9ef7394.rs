use vstd::prelude::*;

verus! {

pub open spec fn gen_pair_fst<A, B>(outputs: Set<(A, B)>) -> Set<A> {
    Set::new(|a: A| exists|b: B| outputs.contains((a, b)))
}


pub proof fn gen_pair_fst_contains<A, B>(outputs: Set<(A, B)>, a: A, b: B)
    requires outputs.contains((a, b))
    ensures gen_pair_fst(outputs).contains(a)
{
}

} // verus!