use vstd::prelude::*;

verus! {

pub open spec fn gen_sorted_list_outputs(max_len: nat, bound: nat) -> Set<Seq<nat>> {
    Set::new(|s: Seq<nat>|
        s.len() <= max_len &&
        (forall|i: int| 0 <= i < s.len() ==> #[trigger] s[i] <= bound) &&
        (forall|i: int, j: int| 0 <= i < j < s.len() ==> s[i] <= s[j])
    )
}


pub proof fn gen_sorted_is_sorted(max_len: nat, bound: nat, s: Seq<nat>, i: int, j: int)
    requires
        gen_sorted_list_outputs(max_len, bound).contains(s),
        0 <= i < j < s.len(),
    ensures s[i] <= s[j]
{
}

} // verus!