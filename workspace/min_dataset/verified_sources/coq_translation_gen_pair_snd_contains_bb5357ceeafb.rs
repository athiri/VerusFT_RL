use vstd::prelude::*;

verus! {

pub open spec fn gen_pair_snd<A, B>(outputs: Set<(A, B)>) -> Set<B> {
    Set::new(|b: B| exists|a: A| outputs.contains((a, b)))
}


pub proof fn gen_pair_snd_contains<A, B>(outputs: Set<(A, B)>, a: A, b: B)
    requires outputs.contains((a, b))
    ensures gen_pair_snd(outputs).contains(b)
{
}

} // verus!