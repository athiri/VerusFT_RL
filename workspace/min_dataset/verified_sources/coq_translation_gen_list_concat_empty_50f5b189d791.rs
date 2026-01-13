use vstd::prelude::*;

verus! {

pub open spec fn gen_list_concat<T>(
    out1: Set<Seq<T>>,
    out2: Set<Seq<T>>
) -> Set<Seq<T>> {
    Set::new(|s: Seq<T>|
        exists|s1: Seq<T>, s2: Seq<T>| out1.contains(s1) && out2.contains(s2) && s == s1 + s2
    )
}

pub open spec fn gen_nil_outputs<T>() -> Set<Seq<T>> {
    set![Seq::empty()]
}


pub proof fn gen_list_concat_empty<T>()
    ensures gen_list_concat(gen_nil_outputs::<T>(), gen_nil_outputs::<T>()).contains(Seq::empty())
{
    let s1: Seq<T> = Seq::empty();
    let s2: Seq<T> = Seq::empty();
    assert(gen_nil_outputs::<T>().contains(s1));
    assert(gen_nil_outputs::<T>().contains(s2));
    assert(s1 + s2 =~= Seq::empty());
}

} // verus!