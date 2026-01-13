use vstd::prelude::*;

verus! {

pub open spec fn gen_option_map<T, U>(
    outputs: Set<Option<T>>,
    f: spec_fn(T) -> U
) -> Set<Option<U>> {
    Set::new(|o: Option<U>| match o {
        Option::None => outputs.contains(Option::None),
        Option::Some(y) => exists|x: T| outputs.contains(Option::Some(x)) && f(x) == y,
    })
}


pub proof fn gen_option_map_none<T, U>(outputs: Set<Option<T>>, f: spec_fn(T) -> U)
    requires outputs.contains(Option::None)
    ensures gen_option_map(outputs, f).contains(Option::None)
{
}

} // verus!