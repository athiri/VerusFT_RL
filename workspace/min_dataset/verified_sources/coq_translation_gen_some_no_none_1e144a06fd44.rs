use vstd::prelude::*;

verus! {

pub open spec fn gen_some_outputs<T>(inner_outputs: Set<T>) -> Set<Option<T>> {
    Set::new(|o: Option<T>| match o {
        Option::None => false,
        Option::Some(x) => inner_outputs.contains(x),
    })
}


pub proof fn gen_some_no_none<T>(inner_outputs: Set<T>)
    ensures !gen_some_outputs(inner_outputs).contains(Option::None)
{
}

} // verus!