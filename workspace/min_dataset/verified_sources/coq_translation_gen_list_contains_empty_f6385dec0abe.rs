use vstd::prelude::*;

verus! {

pub open spec fn gen_list_outputs<T>(inner_outputs: Set<T>, max_len: nat) -> Set<Seq<T>> {
    Set::new(|s: Seq<T>|
        s.len() <= max_len &&
        forall|i: int| 0 <= i < s.len() ==> inner_outputs.contains(#[trigger] s[i])
    )
}


pub proof fn gen_list_contains_empty<T>(inner_outputs: Set<T>, max_len: nat)
    ensures gen_list_outputs(inner_outputs, max_len).contains(Seq::empty())
{
}

} // verus!