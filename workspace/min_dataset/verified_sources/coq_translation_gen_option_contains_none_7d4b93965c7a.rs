use vstd::prelude::*;

verus! {

pub open spec fn gen_option_outputs<T>(inner_outputs: Set<T>) -> Set<Option<T>> {
    Set::new(|o: Option<T>| match o {
        Option::None => true,
        Option::Some(x) => inner_outputs.contains(x),
    })
}


pub proof fn gen_option_contains_none<T>(inner_outputs: Set<T>)
    ensures gen_option_outputs(inner_outputs).contains(Option::None)
{
}

} // verus!