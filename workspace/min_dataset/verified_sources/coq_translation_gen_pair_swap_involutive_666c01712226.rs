use vstd::prelude::*;

verus! {

pub open spec fn gen_pair_swap<A, B>(outputs: Set<(A, B)>) -> Set<(B, A)> {
    Set::new(|p: (B, A)| outputs.contains((p.1, p.0)))
}


pub proof fn gen_pair_swap_involutive<A, B>(outputs: Set<(A, B)>)
    ensures gen_pair_swap(gen_pair_swap(outputs)) =~= outputs
{
    assert forall|p: (A, B)| gen_pair_swap(gen_pair_swap(outputs)).contains(p) <==>
        outputs.contains(p) by {
        // swap(swap(p)) == p
    }
}

} // verus!