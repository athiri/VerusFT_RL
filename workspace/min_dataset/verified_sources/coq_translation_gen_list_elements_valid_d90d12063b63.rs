use vstd::prelude::*;

verus! {

pub open spec fn gen_list_outputs<T>(inner_outputs: Set<T>, max_len: nat) -> Set<Seq<T>> {
    Set::new(|s: Seq<T>|
        s.len() <= max_len &&
        forall|i: int| 0 <= i < s.len() ==> inner_outputs.contains(#[trigger] s[i])
    )
}


pub proof fn gen_list_elements_valid<T>(inner_outputs: Set<T>, max_len: nat, s: Seq<T>, i: int)
    requires
        gen_list_outputs(inner_outputs, max_len).contains(s),
        0 <= i < s.len(),
    ensures inner_outputs.contains(s[i])
{
}

} // verus!