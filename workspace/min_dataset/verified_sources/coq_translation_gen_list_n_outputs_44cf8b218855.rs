use vstd::prelude::*;

verus! {

pub open spec fn gen_list_n_outputs<T>(inner_outputs: Set<T>, n: nat) -> Set<Seq<T>>
    decreases n
{
    if n == 0 {
        set![Seq::empty()]
    } else {
        Set::new(|s: Seq<T>|
            s.len() == n &&
            forall|i: int| 0 <= i < n ==> inner_outputs.contains(#[trigger] s[i])
        )
    }
}

} // verus!