use vstd::prelude::*;

verus! {

pub open spec fn gen_list_outputs<T>(inner_outputs: Set<T>, max_len: nat) -> Set<Seq<T>> {
    Set::new(|s: Seq<T>|
        s.len() <= max_len &&
        forall|i: int| 0 <= i < s.len() ==> inner_outputs.contains(#[trigger] s[i])
    )
}


pub proof fn gen_list_contains_singleton<T>(inner_outputs: Set<T>, max_len: nat, x: T)
    requires inner_outputs.contains(x), max_len >= 1
    ensures gen_list_outputs(inner_outputs, max_len).contains(seq![x])
{
    let s = seq![x];
    assert(s.len() == 1);
    assert(s.len() <= max_len);
    assert forall|i: int| 0 <= i < s.len() implies inner_outputs.contains(#[trigger] s[i]) by {
        assert(s[0] == x);
    }
}

} // verus!