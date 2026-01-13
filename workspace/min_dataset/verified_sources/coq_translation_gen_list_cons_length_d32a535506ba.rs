use vstd::prelude::*;

verus! {

pub open spec fn gen_list_cons<T>(
    elem_outputs: Set<T>,
    list_outputs: Set<Seq<T>>
) -> Set<Seq<T>> {
    Set::new(|s: Seq<T>|
        s.len() > 0 &&
        elem_outputs.contains(s[0]) &&
        list_outputs.contains(s.drop_first())
    )
}


pub proof fn gen_list_cons_length<T>(elem_outputs: Set<T>, list_outputs: Set<Seq<T>>, s: Seq<T>)
    requires gen_list_cons(elem_outputs, list_outputs).contains(s)
    ensures s.len() > 0
{
}

} // verus!