use vstd::prelude::*;

verus! {

pub open spec fn gen_list_map<T, U>(
    outputs: Set<Seq<T>>,
    f: spec_fn(T) -> U
) -> Set<Seq<U>> {
    Set::new(|s: Seq<U>|
        exists|t: Seq<T>| outputs.contains(t) && s.len() == t.len() &&
            forall|i: int| 0 <= i < s.len() ==> s[i] == f(#[trigger] t[i])
    )
}


pub proof fn gen_list_map_length<T, U>(outputs: Set<Seq<T>>, f: spec_fn(T) -> U, s: Seq<U>, t: Seq<T>)
    requires
        outputs.contains(t),
        s.len() == t.len(),
        forall|i: int| 0 <= i < s.len() ==> s[i] == f(#[trigger] t[i]),
    ensures gen_list_map(outputs, f).contains(s)
{
}

} // verus!