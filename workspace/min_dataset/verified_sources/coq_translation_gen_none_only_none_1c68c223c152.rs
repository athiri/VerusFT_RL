use vstd::prelude::*;

verus! {

pub open spec fn gen_none_outputs<T>() -> Set<Option<T>> {
    set![Option::None]
}


pub proof fn gen_none_only_none<T>()
    ensures
        gen_none_outputs::<T>().contains(Option::None),
        forall|o: Option<T>| gen_none_outputs::<T>().contains(o) ==> o.is_none(),
{
}

} // verus!