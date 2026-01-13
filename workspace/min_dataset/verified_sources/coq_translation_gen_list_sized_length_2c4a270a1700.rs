use vstd::prelude::*;

verus! {

pub open spec fn gen_list_sized(size: nat) -> Set<Seq<nat>> {
    Set::new(|s: Seq<nat>|
        s.len() <= size &&
        forall|i: int| 0 <= i < s.len() ==> #[trigger] s[i] <= size
    )
}


pub proof fn gen_list_sized_length(size: nat, s: Seq<nat>)
    requires gen_list_sized(size).contains(s)
    ensures s.len() <= size
{
}

} // verus!