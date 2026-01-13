use vstd::prelude::*;

verus! {

pub open spec fn gen_list_nonempty_outputs<T>(inner_outputs: Set<T>, max_len: nat) -> Set<Seq<T>> {
    Set::new(|s: Seq<T>|
        s.len() >= 1 &&
        s.len() <= max_len &&
        forall|i: int| 0 <= i < s.len() ==> inner_outputs.contains(#[trigger] s[i])
    )
}


pub proof fn gen_list_nonempty_has_element<T>(inner_outputs: Set<T>, max_len: nat, s: Seq<T>)
    requires gen_list_nonempty_outputs(inner_outputs, max_len).contains(s), max_len >= 1
    ensures s.len() >= 1
{
}

} // verus!